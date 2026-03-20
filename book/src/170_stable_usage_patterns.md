# Stable Usage Patterns

This chapter records usage patterns that are safest for downstream consumers.

## Pattern 1: Separate parsing from analysis

A durable integration pattern is:

1. parse with PAC
2. convert the AST into your own analysis model if needed
3. run later semantic or policy logic on that model

This avoids coupling too much of your tool to every detail of PAC’s raw AST layout.

## Pattern 2: Preserve preprocessed source for diagnostics

If you use `driver`, keep `Parse::source` around as long as you may need diagnostics.

That enables:

- mapping spans back to files and lines
- debugging parser failures later
- reproducing failures from stored snapshots

## Pattern 3: Make flavor explicit

Even when defaults are convenient, explicit flavor choices are easier to maintain in tools and
tests.

Prefer:

- `Flavor::StdC11` for strict grammar tests
- `Flavor::GnuC11` when GNU syntax is intentional
- `Flavor::ClangC11` when Clang-specific syntax is intentional

## Pattern 4: Test the syntax you depend on

If your downstream tool depends on a specific syntax family, keep representative tests for it.

Examples:

- function-pointer declarators
- designated initializers
- GNU statement expressions
- inline asm
- availability attributes

## Pattern 5: Treat parse failure as data

A mature integration does not assume every input will parse.
It treats parse failure as a structured, reportable outcome.

That means:

- returning parse diagnostics to the caller
- logging the failing source context when appropriate
- keeping failure fixtures in the test corpus

## Pattern 6: Prefer local traversal hooks

When building analyzers, override the narrowest useful visitor hook instead of one huge catch-all
traversal method.

That makes the analysis easier to maintain as the AST evolves.
