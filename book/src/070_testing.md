# Testing

PAC has two broad testing layers:

- direct parser/API tests in `src/tests`
- corpus-style fixtures under `test/reftests/` and, when present, `test/full_apps/`

## Basic commands

The repository `Makefile` wraps the normal Cargo flow:

```sh
make build
make test
```

Those run:

- `cargo build --release`
- `cargo test`

## Parse API tests

`src/tests/parse_api.rs` checks the public `parse` entry points directly.

Examples covered in the repository include:

- constants
- expressions
- declarations
- statements
- translation units

This layer is useful when:

- adding a new public parser entry point
- fixing a small grammar regression
- documenting a minimal parsing example

## Reference tests

The reftest harness in `src/tests/reftests.rs` reads files from `test/reftests/`.
Each case stores:

- the source snippet
- optional `#pragma` directives that affect parsing
- an expected AST printout between `/*===` and `===*/`

That means reftests verify both:

- whether parsing succeeds
- whether the produced tree matches the expected printer output

## Reftest update workflow

The harness supports `TEST_UPDATE=1` to rewrite expected outputs when printer changes are
intentional.

```sh
TEST_UPDATE=1 cargo test reftests
```

Use that carefully. It is appropriate after deliberate AST or printer changes, not as a substitute
for reviewing diffs.

## Full-app fixtures

The repository includes a full-app harness in `src/tests/full_apps.rs`. It supports fixture
directories with a `fixture.toml` manifest describing:

- `mode`
- `flavor`
- `entry`
- `expected`
- `include_dirs`
- `allow_system_includes`
- `tags`

Supported modes are:

- `translation_unit`
- `driver`
- `preprocessed`

This is the right layer for:

- multi-file examples
- include-path behavior
- external fixture snapshots
- deterministic `.i` inputs

## Filtering larger fixture runs

The full-app runner supports environment filters:

```sh
FULL_APP_FILTER=musl/stdint make test
FULL_APP_TAG=synthetic make test
```

These are useful when debugging one fixture family instead of running the whole corpus.

## Current workspace note

The test harness and README describe `test/full_apps`, but that directory is not present in this
workspace snapshot. The book documents the supported format because the code and README do.

## Adding new tests

A practical progression is:

1. Add a `parse_api` unit test for the exact regression
2. Add a reftest if you need a stable printed-tree expectation
3. Add a full-app fixture if preprocessing or multi-file behavior matters
