---
created_at: 2026-05-10T04:16:01.366983+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Atomic storage to ~/.dispatchd/jobs.json via tmp+rename
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Atomic storage to ~/.dispatchd/jobs.json via tmp+rename

All job state persists to ~/.dispatchd/jobs.json using atomic write semantics: create a temporary file, then atomically rename it to the final path. The tmp file is written to the same directory as the final destination to guarantee atomic rename on a single filesystem. DISPATCHD_HOME env var allows overriding the storage path for testing. This strategy ensures that interrupts or crashes never leave the store in a corrupted state.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
