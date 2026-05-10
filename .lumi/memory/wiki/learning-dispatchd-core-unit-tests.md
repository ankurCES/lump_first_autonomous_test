---
created_at: 2026-05-09T21:08:54.875950+00:00
created_by_agent: memory_save_learning
evidence_session: null
confidence: medium
promoted_at: 2026-05-09T22:59:04.393362+00:00
description: dispatchd-core unit tests
source: memory_save_learning
---
# dispatchd-core unit tests

## tempfile::TempDir pattern for filesystem tests

Use `tempfile::TempDir` (dev-dependency `tempfile = "3"`) to create isolated temporary directories. The dir is cleaned up automatically when the `TempDir` value is dropped. Never point tests at real `~/.dispatchd`.

```rust
fn store_in_tmpdir(dir: &TempDir) -> Store {
    let path = dir.path().join("jobs.json");
    Store { jobs: Vec::new(), next_id: 1, path }
}
```

## StoreData visibility — no changes needed

`StoreData` and its fields (`jobs`, `next_id`) are private (no `pub`). The `#[cfg(test)] mod tests` block lives inside `lib.rs` as a child module of the crate root, so Rust's privacy rules allow it to access all private items from the parent module — including private struct fields. **No `pub(crate)` annotations were needed on `StoreData` or `Store` fields.**

## Which tests caught what

- `test_round_trip` — exercises `add`, `save`, manual reload via `StoreData`, and `list`. Would catch serialisation regressions or id/title mismatches.
- `test_atomic_rename` — asserts `jobs.json.tmp` does not exist after `save()`. Directly validates the atomic write pattern (write-to-tmp, rename).
- `test_monotonic_id` — confirms `next_id` increments strictly; would catch any reset or off-by-one in `add()`.

All three passed on the first run without any code changes beyond adding the test module.

## cargo doc

`cargo doc --no-deps -p dispatchd-core` produced zero warnings. All `pub` items already had `///` doc-comments.
