# Project Layout

This chapter is for contributors and advanced users who want to understand where the parser logic
lives.

## Top-level crate layout

The repository is organized around a small public API surface and several internal support modules.

| Path | Purpose |
| --- | --- |
| `src/lib.rs` | Public module exports |
| `src/driver.rs` | File-based parsing via external preprocessing |
| `src/parse.rs` | Direct fragment parsing API |
| `src/ast/` | AST type definitions |
| `src/visit/` | Recursive visitor functions and trait |
| `src/parser/` | Parser implementation split by grammar area |
| `src/loc.rs` | Preprocessor line-marker location mapping |
| `src/span.rs` | `Span` and `Node<T>` wrappers |
| `src/print.rs` | AST debug printer |
| `src/tests/` | Test harnesses and integration-style parser tests |

## AST and visitor organization

The AST is split into focused files:

- `src/ast/declarations.rs`
- `src/ast/expressions.rs`
- `src/ast/statements.rs`
- `src/ast/extensions.rs`
- `src/ast/lexical.rs`

The visitor layer mirrors that structure in `src/visit/`.

That symmetry is useful:

- if you add a new AST node, you usually need a matching visitor hook
- if you are looking for traversal behavior, the corresponding file is easy to find

## Parser organization

The parser implementation is divided by grammar topics instead of one giant file.
Examples include:

- `translation_units_and_functions.rs`
- `declarations_entry.rs`
- `declarators.rs`
- `statements_iteration_and_jump.rs`
- `casts_and_binary.rs`
- `typeof_and_ts18661.rs`

That split makes grammar work more localized.

## Internal environment handling

Parsing depends on `Env`, which tracks parser state such as known typedef names and enabled syntax
flavor. The public `parse` and `driver` APIs construct the right environment for you.

This matters because some C parses depend on whether an identifier is currently known as a typedef.

## Testing layout

`src/tests/` contains:

- API tests
- reftest harnesses
- larger fixture harnesses
- external/system-header related coverage

When changing parser behavior, expect to touch both narrow tests and corpus-style fixtures.

## Contributor workflow

A good change sequence is:

1. reproduce with the smallest possible `parse::*` input
2. add or update a focused test
3. inspect the tree with `Printer`
4. patch the grammar or AST logic
5. run `make test`
