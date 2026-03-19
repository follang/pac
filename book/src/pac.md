# PAC Reference

PAC is a Rust library for parsing C source into an abstract syntax tree.
It targets C11 and can optionally accept GNU and Clang extensions.

At a high level, PAC supports two workflows:

1. Parse a real C file through a system preprocessor with [`pac::driver`].
2. Parse already-preprocessed text or individual C fragments with [`pac::parse`].

## What PAC gives you

- A typed AST under `pac::ast`
- Source spans for parsed nodes under `pac::span`
- File and line reconstruction for preprocessed input under `pac::loc`
- A recursive visitor API under `pac::visit`
- A tree-style debug printer under `pac::print`

## Pipeline

For file parsing, the normal flow is:

```text
C source file
  -> system preprocessor (gcc / clang)
  -> PAC parser
  -> TranslationUnit AST
```

If you already have preprocessed source, you can skip the external preprocessor and call
`driver::parse_preprocessed` or the lower-level functions in `parse`.

## Public modules

| Module | Purpose |
| --- | --- |
| `driver` | High-level API for parsing files via a C preprocessor |
| `parse` | Direct parsing of expressions, declarations, statements, and translation units |
| `ast` | AST definitions for declarations, expressions, statements, and extensions |
| `visit` | Recursive traversal API for AST consumers |
| `span` | Byte offsets for parsed nodes |
| `loc` | Mapping byte offsets back to source files and lines |
| `print` | Debug-oriented tree printer for ASTs |

## Typical use cases

- Build a linter or analysis tool for C code
- Inspect declarations in headers after preprocessing
- Parse small fragments in tests
- Prototype refactoring or code-search tools
- Compare parser output across standard, GNU, and Clang flavors

## Important mental model

PAC is a parser, not a full compiler frontend. It gives you syntax and source structure.
It does not do semantic analysis, type checking, macro expansion itself, or code generation.

That distinction matters:

- If your input still contains active preprocessor directives or macros, use `driver`.
- If you want symbol resolution or type inference, you need to build that on top of the AST.
- Node spans point into the parsed input text, which for `driver` means the preprocessed source.

## Where to start

- Read [Getting Started](./010_getting_started.md) for the basic setup
- Read [Common Workflows](./015_workflows.md) to choose the right entry point
- Read [Driver API](./020_driver.md) or [Parser API](./030_parser.md) depending on your input
- Read [AST Model](./040_ast.md) before writing analysis code
- Read [Visitor Pattern](./050_visitor.md) if you want to walk the tree
