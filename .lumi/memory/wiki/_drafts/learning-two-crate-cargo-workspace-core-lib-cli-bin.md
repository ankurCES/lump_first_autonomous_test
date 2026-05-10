---
created_at: 2026-05-10T04:16:01.365650+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Two-crate Cargo workspace (core lib, cli bin)
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Two-crate Cargo workspace (core lib, cli bin)

The project is organized as a Cargo workspace at /Users/ankur/code/dispatchd with two crates: dispatchd-core (library) contains the Job/Store API and persistence logic; dispatchd-cli (binary) is the CLI consumer that depends on dispatchd-core. This separation allows the core library to remain independent and reusable while the CLI layer provides the user interface.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
