# Contributing to Ferrox

Thank you for your interest in contributing to Ferrox. We welcome contributions from everyone, whether you are a human developer or an AI agent.

All contributions go through the same review process and must meet the same quality standards.

## Ways to Contribute

- **Report bugs** - Open a GitHub issue describing the problem, steps to reproduce, and expected behavior.
- **Request features** - Open a GitHub issue describing the feature and the motivation behind it.
- **Submit pull requests** - Fix bugs, implement features, improve documentation, or add tests.

## Issues Before Pull Requests

Every pull request must be linked to a GitHub issue.

1. Search existing issues to avoid duplicates.
2. Open a new issue if none exists.
3. Wait for acknowledgment before starting significant work.
4. Reference the issue in your PR (`Fixes #42` or `Closes #42`).

Pull requests created without a corresponding issue may be closed.

## Development Setup

### Prerequisites

- Rust 1.74+ (`rustup update stable`)
- `protobuf-compiler`
  - Ubuntu/Debian: `sudo apt install protobuf-compiler`
  - macOS: `brew install protobuf`
- `pre-commit` — `pip install pre-commit` or `brew install pre-commit`

### Pre-commit hooks

Install once after cloning:

```bash
pre-commit install
```

The hooks run `cargo fmt` and `cargo clippy -- -D warnings` on every commit so issues are caught before they reach CI.

### Build and test

```bash
make build    # debug build
make test     # run all tests
make fmt      # format code
make lint     # clippy
make check    # fmt-check + lint + test (CI equivalent)
```

### Run locally

```bash
cp .env.example .env        # fill in your API keys
make run                    # creates config/local.yaml on first run, then starts the server
```

See [Development](docs/developer/development.md) for more detail.

## Pull Request Guidelines

### Before submitting

- [ ] Linked to a GitHub issue.
- [ ] `cargo test` passes.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy -- -D warnings` passes.
- [ ] Documentation updated if behavior changed.

### Code quality standards

- Follow existing patterns in the codebase.
- One issue per pull request. Keep changes focused.
- Add tests for new functionality and bug fixes.
- No `unwrap()` in non-test code except inside `Lazy::new` metric registrations.
- No `Mutex` or `RwLock` on hot paths. Use atomics.
- Log with structured fields: `tracing::info!(field = %value, "message")`.

### Tests

- Unit tests go in `#[cfg(test)] mod tests` at the bottom of each source file.
- Use `initial_backoff_ms: 0` in retry configs to avoid actual sleeps.
- Test logic and transformations directly. Do not mock HTTP at the unit test level.

### Commit messages

Use the imperative mood. Example: `Add weighted routing strategy` not `Added weighted routing`.

### Documentation

Include documentation updates in the same PR as code changes. This covers:

- New config fields or providers
- Changed API behavior
- New routing strategies or circuit breaker tuning

## For AI Agent Contributors

AI-generated contributions are welcome and go through the same process as human contributions:

1. An issue must exist before a pull request is created.
2. All checks (`cargo test`, `cargo fmt`, `cargo clippy`) must pass.
3. Code must meet the same quality and security standards.

The [Architecture](docs/developer/architecture.md) and [Development](docs/developer/development.md) guides contain project-specific instructions and conventions that AI agents should follow when contributing.

## License

By contributing to Ferrox, you agree that your contributions will be licensed under the [MIT License](LICENSE).
