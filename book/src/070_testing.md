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

## Extraction tests

`src/tests/extraction_fixtures.rs` contains fixture-based tests for the extraction pipeline:
typical C patterns (stdio-style, nested structs, typedef chains, function pointers, etc.).

`src/extract/mod.rs` also contains unit tests for each declaration family.

## Hostile header tests

`src/tests/hostile_headers.rs` covers edge-case and historically problematic C declarations:
deep pointer nesting, anonymous structs/enums, specifier ordering variations, bitfield-only
structs, extreme enum values, forward-then-define patterns, etc.

## Recovery tests

`src/tests/recovery.rs` tests graceful handling of broken, incomplete, or unusual input.
Uses both strict parsing (error expected) and resilient parsing (recovery expected).

## Contract tests

`src/tests/contract.rs` and `src/tests/consumability.rs` verify that the `SourcePackage`
contract is sufficient for downstream consumers. These test iteration patterns, type
navigation, serialization, filtering, merging, and programmatic construction.

## Differential tests

`src/tests/differential.rs` documents the known differences between parc extraction and
bic extraction, ensuring behavioral equivalence on standard declarations and explicitly
documenting intentional divergences (pointer model, no ABI fields, typedef chain
preservation).

## Multi-file scan tests

`src/tests/scan_multifile.rs` covers multi-header scanning scenarios: include chains,
multiple entry headers, cross-file struct references, conditional compilation, include
guards, include directory resolution, and metadata population.

## Adding new tests

A practical progression is:

1. Add a `parse_api` unit test for the exact regression
2. Add a reftest if you need a stable printed-tree expectation
3. Add an extraction test if the issue is about declaration modeling
4. Add a scan test if preprocessing or multi-file behavior matters
5. Add a full-app fixture if the case needs a full filesystem layout
