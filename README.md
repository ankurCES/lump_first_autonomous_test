# dispatchd

A tiny Rust job-queue CLI. Add jobs, list them, mark them done or cancelled. Jobs persist to `~/.dispatchd/jobs.json`.

## Features
- Four commands: `add`, `list`, `cancel`, `complete`
- Priority levels: `high`, `med`, `low`
- Atomic JSON persistence (tmp+rename)
- Pretty table output with ANSI colour (respects `NO_COLOR`)
- `DISPATCHD_HOME` env var overrides the storage directory

## Install
```bash
cargo install --path dispatchd-cli
```

## Usage
```bash
dispatchd add "Fix the bug" --priority high
dispatchd add "Write docs"
dispatchd list
dispatchd list --status all
dispatchd complete 1
dispatchd cancel 2
dispatchd list --status done
```
