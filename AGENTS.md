# AGENTS.md

## Project
Calculates the number of full days, months, and years between two dates.
Stack: Rust (edition 2024), clap + chrono + colored, no database.

## Commands
Run `make help` for the full list. The common ones:
- Full verification: `make check` (fmt → clippy → build → test)
- Run: `make run` or `cargo run -- since DATE`
- Tests only: `make test`
- Release build: `make release`
- Install: `make install`

## Definition of Done
A feature is done only when:
- `make check` is green (including tests that existed before this session).
- The feature's specific verification passes.

NOT done: "code is written", "looks right", "unit tests pass", "works on my machine".

## Hard constraints (MUST / MUST NOT)
1. MUST: all user-facing messages go through `entry_point()` — `main()` only handles the exit code. This keeps error formatting (red `[ERROR]` prefix) consistent and testable.
2. MUST: every visible behavioral change gets an E2E test in `tests/cli.rs`.
3. MUST NOT: commit secrets; all values come from environment variables.

## Work rules
- Match existing code style and patterns.
- No task tracker is configured. Track work in the conversation.

## Escalate to human (stop and ask)
- Changing the CLI interface (adding/removing subcommands or positional arguments).
- New external dependency.
- Any decision affecting security or privacy.
- Deleting anything that cannot be recovered.
