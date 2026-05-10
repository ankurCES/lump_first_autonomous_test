---
created_at: 2026-05-09T21:16:07.432059+00:00
created_by_agent: memory_save_learning
evidence_session: null
confidence: medium
promoted_at: 2026-05-09T22:59:03.019395+00:00
description: dispatchd-cli integration tests and README
source: memory_save_learning
---
# dispatchd-cli integration tests and README

## CARGO_BIN_EXE pattern for integration tests

Integration tests in `dispatchd-cli/tests/` use `env!("CARGO_BIN_EXE_dispatchd")` to locate the compiled binary. Cargo sets `CARGO_BIN_EXE_<name>` automatically when running `cargo test`; no manual path-wrangling needed. The binary name matches the `[[bin]] name` field in `Cargo.toml` (here `"dispatchd"`).

## DISPATCHD_HOME isolation end-to-end

Each integration test creates a `tempfile::TempDir`, passes its path as `DISPATCHD_HOME` when spawning the `dispatchd` binary via `std::process::Command`. The core library's `Store::default_path()` checks `DISPATCHD_HOME` first, so every invocation in the test reads/writes the isolated temp dir. The `TempDir` auto-cleans on drop — no ~/.dispatchd pollution.

```rust
fn run(args: &[&str], home: &str) -> (String, String, i32) {
    let output = Command::new(dispatchd_bin())
        .args(args)
        .env("DISPATCHD_HOME", home)
        .output()
        .expect("failed to run dispatchd");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}
```

## Issues / notes

- `tempfile = "3"` must be added to `[dev-dependencies]` in `dispatchd-cli/Cargo.toml`; it was already present in `dispatchd-core/Cargo.toml` but each crate needs its own declaration.
- Tests in `dispatchd-cli/tests/` are crate-level integration tests; they don't have access to internal types — they test the binary as a black box.
- `cargo test --workspace` result: 1 integration test + 3 unit tests, all green.
