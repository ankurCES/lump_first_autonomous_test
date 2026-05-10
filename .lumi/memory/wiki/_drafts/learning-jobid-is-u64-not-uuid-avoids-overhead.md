---
created_at: 2026-05-10T04:16:01.366237+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: JobId is u64, not UUID (avoids overhead)
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# JobId is u64, not UUID (avoids overhead)

Job IDs are defined as `u64`, not UUID. This is a simple monotonic counter for a local CLI where jobs are ephemeral and don't need global uniqueness. Using u64 avoids UUID overhead and simplifies sorting. From episode decisions: "JobId = u64 (monotonic counter, avoids UUID overhead for local CLI)".

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
