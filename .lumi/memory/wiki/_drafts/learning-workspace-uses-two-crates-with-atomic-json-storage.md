---
created_at: 2026-05-10T01:55:40.915447+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Workspace uses two crates with atomic JSON storage
evidence_episode_ids:
- ea58a3029d36c38c
- e820b1d0b19236e3
model: claude-haiku-4-5
source: semantic_extractor
---
# Workspace uses two crates with atomic JSON storage

The dispatchd project is a Cargo workspace with two crates: dispatchd-core (library) and dispatchd-cli (binary). Job data persists to `~/.dispatchd/jobs.json`. Persistence uses atomic write via temporary file + rename in the same directory, ensuring atomicity on a single filesystem. The `DISPATCHD_HOME` environment variable overrides the storage path, enabling test isolation and user customization.

---

*Distilled from episodes: ea58a3029d36c38c, e820b1d0b19236e3*
