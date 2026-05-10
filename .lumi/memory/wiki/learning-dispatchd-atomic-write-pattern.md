---
created_at: 2026-05-09T21:06:46.901781+00:00
created_by_agent: memory_save_learning
evidence_session: null
confidence: medium
promoted_at: 2026-05-09T22:59:05.026532+00:00
description: dispatchd atomic write pattern
source: memory_save_learning
---
# dispatchd atomic write pattern

## Atomic write via tmp+rename in dispatchd-core

`Store::save()` uses the tmp+rename pattern to ensure the jobs file is never left in a partial state:

```rust
let tmp_path = self.path.with_extension("json.tmp");
std::fs::write(&tmp_path, &json)?;
std::fs::rename(&tmp_path, &self.path)?;
```

### Why the tmp file must be in the same directory
`std::fs::rename` is only atomic when both paths are on the **same filesystem partition**. If the tmp file were in `/tmp` and the target in `~/.dispatchd/`, they could be on different mounts, making rename a cross-device copy+delete — not atomic and not safe. Placing the tmp file next to the target via `self.path.with_extension("json.tmp")` guarantees same-partition placement.

### The `path.with_extension("json.tmp")` idiom
`PathBuf::with_extension` replaces the current extension. For a path like `~/.dispatchd/jobs.json`, calling `.with_extension("json.tmp")` yields `~/.dispatchd/jobs.json.tmp` — same directory, distinct name, easy to identify and clean up.

### Outcome
On a successful rename, no `.tmp` file remains on disk. On a crash mid-write, only the `.tmp` file is corrupt; the original target is untouched.

### Also: `tempfile` in dev-dependencies only
`tempfile = "3"` belongs in `[dev-dependencies]` of `dispatchd-core/Cargo.toml` — it is only needed for tests. Moving it out of `[dependencies]` keeps the production binary lighter.
