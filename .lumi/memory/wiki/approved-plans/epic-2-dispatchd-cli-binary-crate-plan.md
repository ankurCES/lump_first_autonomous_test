---
created_at: 2026-05-09T20:58:19.408104+00:00
created_by_agent: lumi-desktop
confidence: high
description: "User-approved plan: epic-2-dispatchd-cli-binary-crate-plan"
kind: approved-plan
plan_stem: epic-2-dispatchd-cli-binary-crate-plan
epic_id: 1baefc9e-836b-4c5f-b56e-df6fd17fad56
story_count: 4
task_count: 9
approved_at: 2026-05-09T20:58:19.408104+00:00
---

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
