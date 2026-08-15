# AGENTS.md

Rules for any AI coding agent working in this repository. These rules are
mandatory, not suggestions. If a rule below conflicts with a general habit,
this file wins.

## Project

Snitch is a Rust process supervisor and log watcher. It runs multiple
applications as a single managed unit, enforces startup ordering between
them via log pattern matching, centralizes their logs, detects error
patterns via configurable rules, and emits notifications (webhook, web
dashboard) when patterns match.

## Language

- All code, identifiers, comments, commit messages, branch names, and
  documentation are written in English, with no exceptions.
- Conversation with the maintainer may happen in Spanish; none of that
  language reaches the repository.

## Architecture

- Hexagonal architecture (ports and adapters), enforced physically via a
  Cargo workspace, not just by convention:
  - `crates/snitch-domain`: entities, value objects, domain errors, and
    port traits. No dependency on any other crate in this workspace and no
    dependency on async runtimes, HTTP, or storage libraries.
  - `crates/snitch-application`: use cases. Depends only on
    `snitch-domain`. Orchestrates domain objects through ports; never talks
    to a concrete adapter directly.
  - `crates/snitch-adapters`: concrete implementations of the ports
    defined in the domain (OS process supervision, SQLite storage, HTTP
    server, webhook client). Depends on `snitch-domain` and
    `snitch-application`.
  - `crates/snitch-cli`: composition root and binary entrypoint. Wires
    adapters into use cases. Contains no business logic.
- Dependencies only ever point inward: adapters and cli depend on
  application and domain; application depends on domain; domain depends on
  nothing internal. If a change would invert this, redesign it instead of
  adding a shortcut import.
- Prefer small, focused modules over large ones. One primary concept per
  file (e.g. one value object, one use case).

## TDD

- This project is test-driven from the first line of code. No production
  code is written without a failing test first.
- Cycle: red (write a failing test that expresses the behavior) -> green
  (write the minimum code to pass it) -> refactor (clean up without
  changing behavior, tests stay green throughout).
- Domain and application logic: unit tests live inline in the same file
  under `#[cfg(test)] mod tests`, following standard Rust convention.
- Adapters that touch the outside world (process spawning, filesystem,
  network, HTTP): tested through the port's contract, with real I/O
  exercised in integration tests under each crate's `tests/` directory
  where practical, not mocked away entirely.
- A change is not done until `cargo test` is green for the whole
  workspace.

## Commits

- Conventional Commits, written in English.
  `type(scope): summary`, imperative mood, lowercase summary, no trailing
  period.
  Types in use: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`,
  `build`, `ci`, `perf`.
  Scope is the crate or area touched, e.g. `domain`, `application`,
  `adapters`, `cli`, `repo`.
- Commits are atomic: one commit is one traceable unit of change (one
  behavior, one fix, one refactor step). Do not bundle unrelated changes.
  Prefer several small commits over one large one; each commit should
  leave the workspace building and its tests passing.
- Never add co-author trailers, generator attributions, or any reference
  to the tool or model that produced the change. Commit messages contain
  only the change itself.
- No emojis anywhere: not in commit messages, not in code, not in
  comments, not in documentation, not in CLI output, not in the web
  dashboard.

## Branching (Git Flow)

- `main`: always releasable. Only receives merges from `release/*` or
  `hotfix/*` branches.
- `develop`: integration branch. Default base for new work.
- `feature/<short-description>`: branched from `develop`, merged back into
  `develop`. One feature branch per unit of work.
- `release/<version>`: branched from `develop` when preparing a release,
  merged into both `main` and `develop`.
- `hotfix/<short-description>`: branched from `main` for urgent fixes,
  merged into both `main` and `develop`.
- Branch names in English, lowercase, hyphen-separated.
- The `git-flow` CLI extension is not assumed to be installed; the branch
  model is followed manually with plain `git`.

## Code style

- No emojis anywhere in the codebase, output, or docs.
- No AI/model/tool attribution anywhere: not in commit messages, not in
  comments, not in generated files, not in the README.
- Favor explicit, well-named code over clever abstractions. Do not add
  configuration options, traits, or indirection for hypothetical future
  needs; add them when a second real use case demands them.
- Errors are typed (`thiserror` in library crates), not stringly-typed
  panics, except at the composition root boundary where a top-level error
  may be reported and the process exits non-zero.

## Definition of done for any change

1. A failing test existed before the implementation.
2. `cargo test` passes for the whole workspace.
3. `cargo build` succeeds for the whole workspace.
4. No emojis, no co-author or model attribution anywhere in the diff.
5. Commit(s) are atomic and follow Conventional Commits in English.
6. Code, comments, and docs touched by the change are in English.
