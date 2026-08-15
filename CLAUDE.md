# CLAUDE.md

This file guides Claude Code in this repository.

The full set of project rules (architecture, TDD workflow, commit
conventions, branching model, language, and code style) lives in
[AGENTS.md](AGENTS.md) and applies in full to Claude Code as well. Read it
before making any change. Do not duplicate its rules here; if the two ever
disagree, AGENTS.md is the source of truth.

## Quick reference

- Workspace layout: `crates/snitch-domain`, `crates/snitch-application`,
  `crates/snitch-adapters`, `crates/snitch-cli`. Dependencies point inward
  only (domain has none, application depends on domain, adapters/cli
  depend on both).
- Write the failing test before the implementation, every time. No
  exceptions for "small" changes.
- Run `cargo test` for the whole workspace before considering any change
  complete.
- Conventional Commits in English, atomic, no co-author or model
  attribution, no emojis anywhere.
- Git Flow branching: work happens on `feature/*` branches off `develop`;
  `main` only receives `release/*` or `hotfix/*` merges.
- Everything committed to this repository (code, identifiers, comments,
  commit messages, docs) is in English, regardless of what language the
  conversation with the user happens in.
