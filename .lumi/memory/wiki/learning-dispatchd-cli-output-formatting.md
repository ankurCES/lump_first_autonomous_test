---
created_at: 2026-05-09T21:21:18.341453+00:00
created_by_agent: memory_save_learning
evidence_session: null
confidence: medium
promoted_at: 2026-05-09T22:59:01.918937+00:00
description: dispatchd-cli output formatting
source: memory_save_learning
---
# dispatchd-cli output formatting

## comfy-table + colored integration in dispatchd-cli

**Deps added to `dispatchd-cli/Cargo.toml`:**
```toml
comfy-table = "7"
colored = "2"
```

**NO_COLOR / is_terminal() pattern:**
- Use `std::io::IsTerminal` trait (stable since Rust 1.70, no extra dep needed)
- `colour_enabled()` returns `std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal()`
- Import `use std::io::IsTerminal;` at top of file

**comfy-table API notes:**
- `Table::new()` → `table.set_header(vec![...])` → `table.add_row(vec![Cell::new(...)])`
- `table.column_mut(0).unwrap().set_cell_alignment(CellAlignment::Right)` — call after `set_header`, not before; column must exist first
- `Cell::new()` accepts anything `Display`; for coloured strings, pass the `String` directly
- Print with `println!("{table}")` — `Table` implements `Display`

**colored API notes:**
- Import `use colored::Colorize;` to bring `.red()`, `.yellow()`, `.green()` into scope
- `.red().to_string()` converts `ColoredString` back to `String` for storage in `Cell::new()`
- When colour is disabled, fall back to plain `format!("{p:?}")` — don't call `.red()` etc.

**Quirks encountered:**
- None — the API was clean. `is_terminal()` on stdout worked correctly; `NO_COLOR=1 cargo run -- list` produced a plain ASCII box table with no ANSI escape codes.
- `print_jobs_table` handles the empty case inline (`println!("No jobs found.")`) rather than the caller checking, keeping call sites clean.

**Files changed:** `dispatchd-cli/Cargo.toml`, `dispatchd-cli/src/main.rs`
