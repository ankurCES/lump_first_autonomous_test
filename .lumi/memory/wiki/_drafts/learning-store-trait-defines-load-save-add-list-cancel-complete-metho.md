---
created_at: 2026-05-10T02:19:36.123894+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Store trait defines load/save/add/list/cancel/complete methods
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Store trait defines load/save/add/list/cancel/complete methods

The Store trait in dispatchd-core exports six core methods: `load()` (restore from disk), `save()` (write to disk), `add()` (create new job), `list()` (fetch all jobs), `cancel()` (mark cancelled), `complete()` (mark completed). All methods return `anyhow::Result<T>` for consistent error handling.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
