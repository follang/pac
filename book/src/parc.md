# PARC Reference

PARC is the source frontend of the toolchain. The real crate surface today is:

- preprocessing through both external-driver and built-in paths
- C parsing into a typed AST
- extraction into a durable source IR
- header scanning that goes straight to `SourcePackage`
- AST-oriented support APIs such as visiting, spans, locations, and printing

That means the crate serves two audiences at once:

1. downstream tools that want `parc::ir::SourcePackage`
2. parser-facing tools that want direct AST access

## What PARC Owns

- preprocessing
- parsing
- parser recovery
- source extraction
- source diagnostics and provenance
- source IR
- header scanning
- AST traversal and debug support

## What PARC Does Not Own

- symbol inventories
- binary validation
- link-plan construction
- Rust lowering or crate emission

## Actual Data Flow

```text
raw source / headers
  -> driver or built-in preprocessor
  -> parser AST
  -> extraction
  -> SourcePackage
  -> serialized source artifact or downstream harness
```

`scan` short-circuits that flow into one high-level operation. `parse` and
`driver` expose earlier stages for syntax-level consumers.

## Module Layout

| Module | What it is actually for |
| --- | --- |
| `driver` | file-oriented parse flow using an external preprocessor |
| `preprocess` | built-in preprocessing, tokenization, include resolution |
| `parse` | fragment parsing and direct translation-unit parsing from strings |
| `scan` | end-to-end header scanning into `SourcePackage` |
| `extract` | AST-to-IR lowering and normalization |
| `ir` | durable PARC-owned source contract |
| `ast` | syntax tree for parser-facing consumers |
| `visit` | traversal hooks over the AST |
| `span` / `loc` | source-position helpers |
| `print` | debug-oriented AST printer |
| `intake` | already-preprocessed source intake helpers |

## Boundary

The strongest consumer boundary is `parc::ir::SourcePackage`.

That is the point where PARC stops owning the problem. Anything involving
binary evidence or Rust generation is downstream from PARC, even if tests and
harnesses compose those crates together elsewhere.

## Reading Strategy

Read the book in one of these orders:

1. source-contract path:
   `Getting Started -> Source IR -> Extraction -> Header Scanning -> API Contract`
2. parser-facing path:
   `Getting Started -> Driver API -> Parser API -> AST Model -> Visitor Pattern`
3. contributor/debug path:
   `Project Layout -> Testing -> Diagnostics And Printing -> Parser Boundaries`
