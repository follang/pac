# Unsupported Cases

This chapter records the important unsupported or intentionally out-of-scope areas.

The goal is to prevent downstream users from mistaking absence of detail for implicit support.

## Semantic analysis

PAC does not provide:

- full name resolution
- type checking
- constant folding as a stable analysis contract
- ABI or layout proof
- compiler-quality warnings

It is a parser with source-structure support, not a complete compiler frontend.

## Preprocessing

PAC does not implement a standalone C preprocessor in the `driver` path.

Instead it depends on an external preprocessor command such as:

- `gcc -E`
- `clang -E`

That means PAC does not try to normalize every compiler’s preprocessing behavior internally.

## Extension completeness

PAC supports several GNU and Clang extensions, but the project does not promise complete parity
with every extension accepted by modern GCC or Clang releases.

Downstream tools should not assume:

- full GNU extension completeness
- full Clang extension completeness
- identical acceptance behavior across all compiler-version-specific syntax edges

## Macro inventory and expansion modeling

PAC parses the post-preprocessing result. It does not expose a first-class macro inventory or a
stable semantic model of macro definitions as its own output contract.

If you need macro capture as data, that is outside PAC’s current scope.

## Translation-unit semantics

PAC can parse translation units, but it does not guarantee:

- cross-file symbol resolution
- duplicate-definition analysis as a stable feature
- semantic correctness of declarations
- linkability of parsed declarations

Those tasks belong to later analysis layers, not the parser itself.

## Diagnostics depth

PAC does not currently provide:

- warning classes
- fix-it suggestions
- rich categorized error codes
- a stable diagnostic JSON schema

The current error model is strong enough for syntax handling, not full compiler UX.

## Consumer guidance

Downstream tools should treat these gaps as explicit non-guarantees.

That means:

- build policy around syntax success and failure, not semantic certainty
- isolate extension-heavy assumptions behind tests
- keep representative preprocessed fixtures for any hard parser dependency
