---
created_at: 2026-05-10T04:16:01.367834+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Integration tests use tempfile for store isolation
evidence_episode_ids:
- e820b1d0b19236e3
model: claude-haiku-4-5
source: semantic_extractor
---
# Integration tests use tempfile for store isolation

End-to-end integration tests create isolated job stores via `tempfile::TempDir` and override the DISPATCHD_HOME env var to point to temporary directories. This allows tests to run in parallel without shared state conflicts. All tests are in the workspace and must pass via `cargo test --workspace` before commits.

---

*Distilled from episodes: e820b1d0b19236e3*
