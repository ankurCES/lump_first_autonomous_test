---
created_at: 2026-05-10T02:19:36.123642+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Store persists jobs to ~/.dispatchd/jobs.json via atomic tmp+rename
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Store persists jobs to ~/.dispatchd/jobs.json via atomic tmp+rename

Jobs persist to `~/.dispatchd/jobs.json` using atomic write semantics: write to a temporary file in the same directory as the target, then atomically rename to the final path. This ensures no partial writes corrupt the database if the process crashes mid-write. The `DISPATCHD_HOME` environment variable can override the default storage location for test isolation.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
