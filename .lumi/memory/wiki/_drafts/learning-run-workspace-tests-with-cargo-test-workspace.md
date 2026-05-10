---
created_at: 2026-05-10T01:55:40.916815+00:00
created_by_agent: memory-semantic-extractor
evidence_session: null
confidence: medium
promoted_at: null
description: Run workspace tests with 'cargo test --workspace'
evidence_episode_ids:
- e820b1d0b19236e3
- 21f9dcb8b172c906
model: claude-haiku-4-5
source: semantic_extractor
---
# Run workspace tests with 'cargo test --workspace'

Use `cargo test --workspace` to run all tests, which passes cleanly for both dispatchd-core (library) and dispatchd-cli (binary). The workspace can also be verified with `cargo check`. This ensures full end-to-end coverage across both crates.

---

*Distilled from episodes: e820b1d0b19236e3, 21f9dcb8b172c906*
