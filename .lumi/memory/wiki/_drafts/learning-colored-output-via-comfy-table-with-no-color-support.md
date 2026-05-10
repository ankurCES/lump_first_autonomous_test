---
created_at: 2026-05-10T04:16:01.367641+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Colored output via comfy-table with NO_COLOR support
evidence_episode_ids:
- e820b1d0b19236e3
- ea58a3029d36c38c
model: claude-haiku-4-5
source: semantic_extractor
---
# Colored output via comfy-table with NO_COLOR support

Job tables are pretty-printed using comfy-table with priority-based ANSI coloring from the colored crate: high priority = red, medium = yellow, low = green. The implementation respects the NO_COLOR env var and uses `is_terminal()` to detect interactive contexts, avoiding forced colors in pipes, redirects, or accessibility scenarios.

---

*Distilled from episodes: e820b1d0b19236e3, ea58a3029d36c38c*
