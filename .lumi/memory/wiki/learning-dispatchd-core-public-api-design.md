---
created_at: 2026-05-09T21:05:24.099267+00:00
created_by_agent: memory_save_learning
evidence_session: null
confidence: medium
promoted_at: 2026-05-10T00:19:42.631194+00:00
description: dispatchd-core public API design
source: memory_save_learning
---
# dispatchd-core public API design

## Key type decisions for dispatchd-core (lib.rs)

- **`JobId = u64`** — simple monotonic counter; avoids UUID overhead for a local CLI tool. `next_id` is persisted alongside `jobs` in the JSON envelope so it survives restarts.
- **`chrono::DateTime<Utc>`** for `created_at` — serde feature enabled in Cargo.toml (`chrono = { version = "0.4", features = ["serde"] }`), round-trips cleanly as ISO-8601 strings in JSON.
- **`anyhow` for errors** — used only in the `Store` methods (binary-facing API); library-internal helpers stay infallible or return `anyhow::Result`. No `unwrap()`/`expect()` in lib code.
- **`DISPATCHD_HOME` env var pattern** — `Store::default_path()` checks `std::env::var("DISPATCHD_HOME")` first, falls back to `$HOME/.dispatchd/jobs.json`. This makes integration tests trivially hermetic via `DISPATCHD_HOME=<tempdir>`.
- **Internal `StoreData` envelope** — separates the on-disk JSON shape (`{ jobs, next_id }`) from the `Store` struct, keeping `Store` free of `Serialize`/`Deserialize` derives (it holds mutable state, not pure data).
- **No external `dirs` crate** — `HOME` env var resolved manually via `std::env::var("HOME")` to avoid a dependency just for one path lookup.

