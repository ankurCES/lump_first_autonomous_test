---
created_at: 2026-05-10T01:55:40.916143+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Jobs use u64 IDs with Status/Priority enums and Store trait
evidence_episode_ids:
- ea58a3029d36c38c
- e820b1d0b19236e3
model: claude-haiku-4-5
source: semantic_extractor
---
# Jobs use u64 IDs with Status/Priority enums and Store trait

Each Job is identified by a u64 JobId (monotonic counter; avoids UUID overhead for local CLI). The Job struct includes `chrono::DateTime<Utc> created_at` for timestamps. Status and Priority are Rust enums. The Store trait defines: `load()`, `save()`, `add()`, `list()`, `cancel()`, and `complete()` methods. All Store method errors use `anyhow::Result<T>` for simple error propagation.

---

*Distilled from episodes: ea58a3029d36c38c, e820b1d0b19236e3*
