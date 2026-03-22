# Release Checklist

This chapter is a pragmatic checklist for documentation and parser changes before a release.

The important release posture is architectural:

- `parc` releases source/frontend behavior
- it does not release binary or Rust-generation policy
- the tested `SourcePackage` contract matters more than parser-internal churn

## Parser changes

Before releasing parser changes:

1. confirm the smallest reproducer has a test
2. confirm the intended flavor coverage is tested
3. confirm the AST shape change is deliberate
4. confirm visitor and printer behavior still make sense

## Book changes

Before releasing documentation changes:

1. confirm the affected public behavior is described in the book
2. confirm unsupported or out-of-scope cases are still documented honestly
3. confirm examples still match the actual public API names

## Error-surface changes

Before releasing changes around errors:

1. confirm structured fields still provide the needed information
2. avoid treating formatted strings as the real contract
3. update the error-surface chapter if the practical behavior changed

## Workflow changes

Before releasing changes to the normal integration path:

1. update the workflow chapter
2. update the API contract chapter if the preferred boundary changed
3. update stable-usage guidance if downstream posture should change

## Artifact contract changes

Before releasing a `SourcePackage` shape change:

1. confirm the changed field meaning is covered by contract-level tests
2. confirm the consuming workflow examples still describe artifact boundaries
3. confirm cross-crate composition is still described as tests/examples/harness
   work, not library coupling

## Release gate

`parc` is ready to release only when:

- `make build` passes
- `make test` passes
- the canonical hardening surfaces are still green
  - vendored musl `stdint`
  - vendored zlib
  - vendored libpng scan
  - OpenSSL public wrapper extraction
  - combined Linux event-loop wrapper extraction
- deterministic repeated extraction still holds on the canonical large surfaces
- the book still teaches `parc` as the source-meaning crate
- unsupported or partial source behavior is still documented honestly

## Final practical rule

If a change would force a downstream PARC consumer to rethink how it parses, traverses, or reports
on source, the book should say so explicitly in the same change.
