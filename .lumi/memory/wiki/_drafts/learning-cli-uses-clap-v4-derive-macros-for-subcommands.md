---
created_at: 2026-05-10T04:16:01.367425+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: CLI uses clap v4 derive macros for subcommands
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# CLI uses clap v4 derive macros for subcommands

The dispatchd-cli binary uses clap v4's derive macros for minimal boilerplate and automatic subcommand routing. Four subcommands (add, list, cancel, complete) map directly to Store methods. The derive approach allows clap to handle subcommand parsing, help text generation, and validation automatically without manual wiring.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
