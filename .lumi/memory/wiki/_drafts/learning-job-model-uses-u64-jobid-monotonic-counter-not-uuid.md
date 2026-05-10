---
created_at: 2026-05-10T02:19:36.124124+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Job model uses u64 JobId (monotonic counter, not UUID)
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Job model uses u64 JobId (monotonic counter, not UUID)

Jobs use `JobId` as a `u64` monotonic counter instead of UUID, avoiding UUID overhead in a local CLI context while providing unique, predictable identifiers. Each Job struct includes `Status` and `Priority` enums plus a `chrono::DateTime<Utc> created_at` timestamp.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
