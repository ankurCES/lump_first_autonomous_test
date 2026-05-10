---
created_at: 2026-05-10T04:16:01.367213+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: 'Store trait: load, save, add, list, cancel, complete'
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Store trait: load, save, add, list, cancel, complete

The core Store trait defines six methods: `load()` (read all jobs from disk), `save()` (persist all jobs), `add(job)` (insert new job), `list()` (return all jobs), `cancel(id)` (mark cancelled), `complete(id)` (mark done). All methods return `anyhow::Result` for error handling. The JSON file at ~/.dispatchd/jobs.json is the sole backing store.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
