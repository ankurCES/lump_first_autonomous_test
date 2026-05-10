---
created_at: 2026-05-10T01:55:40.916601+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Tests isolate storage with tempfile and env override
evidence_episode_ids:
- e820b1d0b19236e3
model: claude-haiku-4-5
source: semantic_extractor
---
# Tests isolate storage with tempfile and env override

Integration tests use `tempfile::TempDir` (in [dev-dependencies]) to create isolated temporary directories. The `DISPATCHD_HOME` environment variable points tests to the temp directory, preventing test runs from affecting the user's real `~/.dispatchd` directory. This pattern ensures clean test isolation without side effects.

---

*Distilled from episodes: e820b1d0b19236e3*
