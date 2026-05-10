---
created_at: 2026-05-10T02:19:36.122044+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: 'Two-crate workspace: dispatchd-core (lib) and dispatchd-cli (bin)'
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Two-crate workspace: dispatchd-core (lib) and dispatchd-cli (bin)

The dispatchd project uses a two-crate Cargo workspace that separates the job-queue library (dispatchd-core) from the CLI binary consumer (dispatchd-cli). This architecture enables the core storage and job logic to be independently tested and potentially reused outside the CLI context.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
