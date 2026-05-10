---
created_at: 2026-05-10T02:19:36.124332+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: CLI uses clap v4 derives, comfy-table output, priority colors with NO_COLOR
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# CLI uses clap v4 derives, comfy-table output, priority colors with NO_COLOR

dispatchd-cli implements subcommands (add, list, cancel, complete) via clap v4 derive macros. Job tables output via `comfy-table` with priority-based ANSI colors: high=red, medium=yellow, low=green. Implementation respects `NO_COLOR` environment variable and uses `is_terminal()` detection to avoid forcing colors in piped output.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
