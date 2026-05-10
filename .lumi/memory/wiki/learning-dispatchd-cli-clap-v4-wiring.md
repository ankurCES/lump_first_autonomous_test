---
created_at: 2026-05-09T21:12:22.023185+00:00
created_by_agent: memory_save_learning
evidence_session: null
confidence: medium
promoted_at: 2026-05-09T22:59:03.778033+00:00
description: dispatchd-cli clap v4 wiring
source: memory_save_learning
---
# dispatchd-cli clap v4 wiring

## Derive-macro structure

`dispatchd-cli/src/main.rs` uses clap v4 derive macros throughout:

```rust
#[derive(Parser)]          // top-level Cli struct
#[derive(Subcommand)]      // Commands enum
#[derive(Clone, ValueEnum)] // PriorityCli and StatusCli
```

`[[bin]]` with `path = "src/main.rs"` is required in `Cargo.toml` when the package name (`dispatchd`) differs from the binary name.

## PriorityCli / StatusCli → core type mapping

Both CLI enums are separate from the core types (to keep library/binary concerns separate). Conversion is done via `From<PriorityCli> for Priority` (a clean trait impl). `StatusCli` has an `All` variant that maps to `Option<Status>::None` via a `to_filter()` method — it cannot implement `From<StatusCli> for Option<Status>` as cleanly due to the orphan rule, so a plain method was used instead.

## Surprises / gotchas

- **No `Display` needed for `{:?}` in list output** — since `Status` and `Priority` already derive `Debug`, using `{:?}` gives readable output for the simple list view without needing extra derives or formatting boilerplate. Story 2-B will replace this with proper table formatting.
- **`format!("{:?}", job.priority)`** was needed inside `println!` to get a `{:<10}` width-padded string from a Debug-printed enum — you can't width-pad a `{:?}` format directly.
- **Workspace Cargo.toml**: `members = ["dispatchd-core", "dispatchd-cli"]` — order matters for error messages but not compilation.
- **`cargo run -p dispatchd`** uses the package name (`dispatchd`), not the binary name, for `-p`.
- `Store::load()` → mutate → `Store::save()` pattern is enforced at the call site; the library never auto-saves.
