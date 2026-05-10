# EPIC 1 — dispatchd-core library crate

## Objective
Build the `dispatchd-core` Rust library crate providing job queue persistence.

## Public API
- `JobId = u64`
- `Status` enum: `Open`, `Done`, `Cancelled`
- `Priority` enum: `High`, `Med`, `Low`
- `Job` struct: `id`, `title`, `priority`, `status`, `created_at`
- `Store`: `load()`, `save()`, `add(job)`, `list(filter)`, `cancel(id)`, `complete(id)`

## Persistence
- Path: `~/.dispatchd/jobs.json` (overridable via `DISPATCHD_HOME` env var)
- Atomic write: write to `<path>.tmp`, rename to final path

## Tests
- `test_round_trip`: add 3 jobs, save, reload, assert ids + titles
- `test_atomic_rename`: no `.tmp` left after save
- `test_monotonic_id`: successive ids strictly increase
