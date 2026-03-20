# Reproducibility

Parsing C is sensitive to the exact preprocessor environment.

This chapter documents how to keep PAC-based workflows reproducible.

## Main reproducibility risks

The biggest sources of drift are:

- different preprocessor executables
- different default include paths
- different predefined macros
- different parser flavor settings
- different preprocessed snapshots in tests

## Best practices

For durable automation:

1. prefer explicit `Config` values over ambient defaults in CI
2. pin include paths with `-I...` when they matter
3. use `-nostdinc` for isolated fixture testing when appropriate
4. keep preprocessed snapshots for hard parser regressions
5. keep the parser flavor explicit in tests

## Deterministic parse debugging

If a real file parse is inconsistent across machines, a strong debugging move is:

1. capture the preprocessed output
2. switch the failing test to `parse_preprocessed`
3. debug PAC against the stable snapshot

That separates:

- preprocessing differences
- parser differences

## Reftests and snapshots

The reftest harness already encourages deterministic expectations by comparing against printed AST
output. For parser bugs that depend on preprocessing, a pinned `.i` file is often even better.

## Consumer guidance

If PAC is part of a larger pipeline, keep the following recorded somewhere durable:

- preprocessor executable
- preprocessor arguments
- flavor
- representative fixtures
- expected parse outcome

Without that context, debugging parser regressions is much slower.
