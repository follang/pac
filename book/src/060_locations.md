# Location Tracking

PAC tracks source positions in two related ways:

- `Span` stores byte offsets into the parsed input
- `loc` maps byte offsets in preprocessed source back to original files and lines

## `Span`

`Span` is a byte range:

```rust
pub struct Span {
    pub start: usize,
    pub end: usize,
}
```

Most AST values are wrapped in `Node<T>`, which adds a `span` field:

```rust
pub struct Node<T> {
    pub node: T,
    pub span: Span,
}
```

## What spans point to

This depends on the API you used:

- with `parse::*`, spans refer to the string you passed in
- with `driver::parse_preprocessed`, spans refer to the preprocessed string you passed in
- with `driver::parse`, spans refer to the preprocessor output stored in `Parse::source`

That last case is important: spans do not directly point into the original `.c` file when
preprocessing has inserted line markers or expanded includes.

## Mapping offsets back to files

The `loc` module reads preprocessor line markers like:

```text
# 42 "include/header.h" 1
```

From those markers, `get_location_for_offset` reconstructs:

- the active file
- the active line number
- the include stack

## Basic example

```rust
use pac::loc::get_location_for_offset;

let src = "# 1 \"main.c\"\nint value;\n";
let (loc, includes) = get_location_for_offset(src, 18);

assert_eq!(loc.file, "main.c");
assert!(includes.is_empty());
```

## Using spans with locations

The common pattern is:

1. take a node span
2. use `span.start` or `span.end`
3. map that offset through `loc`

Example:

```rust
use pac::driver::{parse, Config};
use pac::loc::get_location_for_offset;

let parsed = parse(&Config::default(), "examples/sample.c")?;

if let Some(first) = parsed.unit.0.first() {
    let (loc, include_stack) = get_location_for_offset(&parsed.source, first.span.start);
    println!("first item starts in {}:{}", loc.file, loc.line);
    println!("include depth: {}", include_stack.len());
}
# Ok::<(), pac::driver::Error>(())
```

## `SyntaxError::get_location`

For parser failures in the driver path, `driver::SyntaxError` already exposes:

```rust
err.get_location()
```

That returns:

- the active source location
- the include chain that led there

This is the best starting point for user-facing diagnostics.

## Caveat: byte offsets, not columns in UTF-16

PAC stores Rust byte offsets. That is usually what you want for source processing, but if you are
feeding results into another tool that expects a different coordinate system, convert explicitly.
