# EPIC 2 — dispatchd-cli binary crate

## Objective
Build the `dispatchd-cli` binary crate consuming `dispatchd-core`.

## Commands
- `dispatchd add <title> [--priority high|med|low]`
- `dispatchd list [--status open|done|cancelled|all]`
- `dispatchd cancel <id>`
- `dispatchd complete <id>`

## Output
- Table format: `ID | Title | Priority | Status`
- ANSI colours: high=red, med=yellow, low=green
- `NO_COLOR` + non-tty disables colour

## Tests
- Integration test via `DISPATCHD_HOME` env override
- `cargo test --workspace` all green
