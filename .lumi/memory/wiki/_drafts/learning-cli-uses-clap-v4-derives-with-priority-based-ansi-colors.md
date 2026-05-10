---
created_at: 2026-05-10T01:55:40.916386+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: CLI uses clap v4 derives with priority-based ANSI colors
evidence_episode_ids:
- ea58a3029d36c38c
- e820b1d0b19236e3
model: claude-haiku-4-5
source: semantic_extractor
---
# CLI uses clap v4 derives with priority-based ANSI colors

The dispatchd-cli binary uses clap v4 derive macros for subcommand routing: add, list, cancel, complete. Job tables are pretty-printed via `comfy-table`. Priority-based coloring: high→red, medium→yellow, low→green. Both `NO_COLOR` environment variable and `is_terminal()` detection are implemented to respect user preferences and terminal capabilities. The `colored` crate provides ANSI output.

---

*Distilled from episodes: ea58a3029d36c38c, e820b1d0b19236e3*
