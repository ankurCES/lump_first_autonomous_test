use std::path::PathBuf;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for a job.
pub type JobId = u64;

/// Lifecycle status of a job.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Open,
    Done,
    Cancelled,
}

/// Scheduling priority of a job.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Med,
    Low,
}

/// A single unit of work tracked by dispatchd.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

/// Persistent job queue backed by a JSON file.
pub struct Store {
    jobs: Vec<Job>,
    next_id: JobId,
    path: PathBuf,
}

/// Internal envelope used for JSON serialisation.
#[derive(Serialize, Deserialize)]
struct StoreData {
    jobs: Vec<Job>,
    next_id: JobId,
}

impl Store {
    /// Returns the path to the jobs file.
    ///
    /// Uses `$DISPATCHD_HOME/jobs.json` when the env var is set, otherwise
    /// falls back to `~/.dispatchd/jobs.json`.
    pub fn default_path() -> PathBuf {
        if let Ok(home) = std::env::var("DISPATCHD_HOME") {
            PathBuf::from(home).join("jobs.json")
        } else {
            let home = dirs_home();
            home.join(".dispatchd").join("jobs.json")
        }
    }

    /// Loads the store from [`default_path`](Self::default_path).
    ///
    /// Returns an empty store when the file does not exist yet.
    pub fn load() -> anyhow::Result<Store> {
        let path = Self::default_path();
        if !path.exists() {
            return Ok(Store { jobs: Vec::new(), next_id: 1, path });
        }
        let file = std::fs::File::open(&path)
            .with_context(|| format!("opening {}", path.display()))?;
        let data: StoreData = serde_json::from_reader(file)
            .with_context(|| format!("parsing {}", path.display()))?;
        Ok(Store { jobs: data.jobs, next_id: data.next_id, path })
    }

    /// Persists the store to disk using an atomic tmp+rename write.
    ///
    /// Writes to `<path>.tmp` first (same directory = same filesystem partition),
    /// then renames to the final path. On success no `.tmp` file remains.
    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating dir {}", parent.display()))?;
        }
        let data = StoreData { jobs: self.jobs.clone(), next_id: self.next_id };
        let json = serde_json::to_string_pretty(&data)?;

        // Write to a .tmp file in the SAME directory (same filesystem → rename is atomic)
        let tmp_path = self.path.with_extension("json.tmp");
        std::fs::write(&tmp_path, &json)
            .with_context(|| format!("writing tmp file {}", tmp_path.display()))?;
        std::fs::rename(&tmp_path, &self.path)
            .with_context(|| format!("renaming {} to {}", tmp_path.display(), self.path.display()))?;
        Ok(())
    }

    /// Adds a new job and returns its assigned [`JobId`].
    pub fn add(&mut self, title: String, priority: Priority) -> JobId {
        let id = self.next_id;
        self.next_id += 1;
        self.jobs.push(Job {
            id,
            title,
            priority,
            status: Status::Open,
            created_at: Utc::now(),
        });
        id
    }

    /// Returns jobs matching `filter`, or all jobs when `filter` is `None`.
    pub fn list(&self, filter: Option<Status>) -> Vec<&Job> {
        self.jobs
            .iter()
            .filter(|j| match &filter {
                None => true,
                Some(s) => &j.status == s,
            })
            .collect()
    }

    /// Marks the job with the given `id` as [`Status::Cancelled`].
    ///
    /// Returns an error if no job with that id exists.
    pub fn cancel(&mut self, id: JobId) -> anyhow::Result<()> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| anyhow!("job {id} not found"))?;
        job.status = Status::Cancelled;
        Ok(())
    }

    /// Marks the job with the given `id` as [`Status::Done`].
    ///
    /// Returns an error if no job with that id exists.
    pub fn complete(&mut self, id: JobId) -> anyhow::Result<()> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| anyhow!("job {id} not found"))?;
        job.status = Status::Done;
        Ok(())
    }
}

/// Resolves `$HOME` without pulling in an extra crate.
fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn store_in_tmpdir(dir: &TempDir) -> Store {
        let path = dir.path().join("jobs.json");
        // Build a Store with a custom path (not default_path)
        Store { jobs: Vec::new(), next_id: 1, path }
    }

    #[test]
    fn test_round_trip() {
        let dir = TempDir::new().unwrap();
        let mut store = store_in_tmpdir(&dir);
        store.add("alpha".into(), Priority::High);
        store.add("beta".into(), Priority::Med);
        store.add("gamma".into(), Priority::Low);
        store.save().unwrap();

        // Reload and verify
        let path = dir.path().join("jobs.json");
        let file = std::fs::File::open(&path).expect("jobs.json must exist after save");
        let data: StoreData = serde_json::from_reader(file).expect("jobs.json must be valid JSON");
        let reloaded = Store { jobs: data.jobs, next_id: data.next_id, path };

        let jobs = reloaded.list(None);
        assert_eq!(jobs.len(), 3, "expected 3 jobs after reload");
        assert_eq!(jobs[0].title, "alpha");
        assert_eq!(jobs[1].title, "beta");
        assert_eq!(jobs[2].title, "gamma");
        assert_eq!(jobs[0].id, 1);
        assert_eq!(jobs[1].id, 2);
        assert_eq!(jobs[2].id, 3);
    }

    #[test]
    fn test_atomic_rename() {
        let dir = TempDir::new().unwrap();
        let mut store = store_in_tmpdir(&dir);
        store.add("task".into(), Priority::Low);
        store.save().unwrap();

        // No .tmp file should remain
        let tmp = dir.path().join("jobs.json.tmp");
        assert!(!tmp.exists(), ".tmp file must not exist after save");
    }

    #[test]
    fn test_monotonic_id() {
        let dir = TempDir::new().unwrap();
        let mut store = store_in_tmpdir(&dir);
        let id1 = store.add("first".into(), Priority::High);
        let id2 = store.add("second".into(), Priority::Med);
        let id3 = store.add("third".into(), Priority::Low);
        assert!(id1 < id2, "ids must increase: {} < {}", id1, id2);
        assert!(id2 < id3, "ids must increase: {} < {}", id2, id3);
    }
}
