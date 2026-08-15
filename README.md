# Snitch

Snitch is a process supervisor and log watcher written in Rust.

It runs several applications as a single managed unit instead of leaving
each one to be started, watched, and diagnosed by hand. It enforces
startup ordering between processes based on log output, centralizes their
logs, detects error patterns using configurable rules, and emits
notifications through webhooks and a web dashboard when a pattern
matches.

## Status

Early development. The domain model and core supervision behavior are
being built incrementally through test-driven development.

## Architecture

Snitch follows a hexagonal architecture, implemented as a Cargo workspace:

- `crates/snitch-domain`: entities, value objects, and ports. No
  dependency on any other crate in this workspace.
- `crates/snitch-application`: use cases orchestrating the domain through
  its ports.
- `crates/snitch-adapters`: concrete adapters (process supervision,
  storage, HTTP server, webhooks).
- `crates/snitch-cli`: binary entrypoint and composition root.

See [AGENTS.md](AGENTS.md) for the full set of contribution rules
(TDD workflow, commit conventions, branching model).

## Building

```
cargo build
```

## Testing

```
cargo test
```

## License

MIT, see [LICENSE](LICENSE).
