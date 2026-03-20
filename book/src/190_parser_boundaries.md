# Parser Boundaries

This chapter explains where PAC starts and where it intentionally stops.

## PAC owns syntax parsing

PAC is responsible for:

- accepting supported C syntax
- building an AST
- carrying spans
- mapping parse positions back through preprocessor line markers

That is the core boundary of the crate.

## PAC does not own full compilation

PAC does not attempt to be:

- a full preprocessor implementation
- a type checker
- a linker-aware analyzer
- a code generator
- a full semantic compiler frontend

These are not accidental omissions. They are part of the intended scope boundary.

## Practical layering

A healthy toolchain boundary looks like this:

1. a compiler or preprocessor produces acceptable input
2. PAC parses it
3. a later layer performs semantic analysis, policy checks, or code generation

This keeps PAC focused on syntax and source structure.

## Why this matters for consumers

If a downstream tool needs:

- ABI guarantees
- linker truth
- semantic type equivalence
- macro inventories as data

then PAC should be one component in the pipeline, not the whole pipeline.

## Why this matters for contributors

When deciding whether a new feature belongs in PAC, a useful question is:

"Does this improve PAC’s syntax parsing and source-structure contract, or does it drag PAC into a
later compiler stage?"

If it is mostly a later-stage concern, it probably belongs outside PAC.
