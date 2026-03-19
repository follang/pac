# Parser API

The `parse` module exposes direct parsing functions that work on in-memory strings. Unlike
`driver`, it does not invoke an external preprocessor.

## Available entry points

```rust
parse::constant(source, flavor)
parse::expression(source, flavor)
parse::declaration(source, flavor)
parse::statement(source, flavor)
parse::translation_unit(source, flavor)
```

These map to progressively larger grammar fragments.

## Return types

The direct parser returns the same `ParseResult<T>` shape for every entry point:

```rust
type ParseResult<T> = Result<T, ParseError>;
```

`ParseError` contains:

- `line`
- `column`
- `offset`
- `expected`

That makes it well suited for parser tests and editor integrations.

## Parse an expression

```rust
use pac::driver::Flavor;
use pac::parse;

let expr = parse::expression("value + 1 * scale", Flavor::StdC11)?;
println!("{:#?}", expr);
# Ok::<(), pac::parse::ParseError>(())
```

The return type is `Box<Node<Expression>>`, so you get both the expression and its span.

## Parse a declaration

```rust
use pac::driver::Flavor;
use pac::parse;

let decl = parse::declaration(
    "static const unsigned long mask = 0xff;",
    Flavor::StdC11,
)?;

println!("{:#?}", decl.node);
# Ok::<(), pac::parse::ParseError>(())
```

Declarations are useful when you want to inspect:

- storage class
- type qualifiers
- declarator structure
- initializers

## Parse a statement

```rust
use pac::driver::Flavor;
use pac::parse;

let stmt = parse::statement(
    "for (int i = 0; i < 4; i++) total += i;",
    Flavor::StdC11,
)?;

println!("{:#?}", stmt.node);
# Ok::<(), pac::parse::ParseError>(())
```

## Parse a whole translation unit

```rust
use pac::driver::Flavor;
use pac::parse;

let source = r#"
typedef int count_t;
count_t inc(count_t x) { return x + 1; }
"#;

let unit = parse::translation_unit(source, Flavor::StdC11)?;
println!("items: {}", unit.0.len());
# Ok::<(), pac::parse::ParseError>(())
```

## Flavor-sensitive parsing

GNU or Clang syntax only parses when you select a compatible flavor.

```rust
use pac::driver::Flavor;
use pac::parse;

let gnu_expr = "({ int x = 1; x + 2; })";
assert!(parse::expression(gnu_expr, Flavor::GnuC11).is_ok());
assert!(parse::expression(gnu_expr, Flavor::StdC11).is_err());
```

## When to prefer `parse`

Use `parse` when:

- you already have a string in memory
- you are testing grammar behavior directly
- you are parsing snippets, not full files
- you want a deterministic input without shelling out to `gcc` or `clang`

Use `driver` instead when preprocessing is part of the problem.
