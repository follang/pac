# Common Workflows

Most confusion with PAC comes from choosing the wrong entry point. This chapter maps common tasks to
the right API.

## Workflow selection

| Situation | API |
| --- | --- |
| Parse a `.c` or `.h` file with includes and macros | `driver::parse` |
| Parse already-preprocessed text from memory | `driver::parse_preprocessed` |
| Parse one expression, declaration, statement, or translation unit string | `parse::*` |
| Walk an AST you already parsed | `visit` |
| Print an AST for debugging | `print::Printer` |

## Parse a real file

Use this when your source depends on `#include`, `#define`, or compiler predefined macros.

```rust
use parc::driver::{parse, Config};

let config = Config::default();
let parsed = parse(&config, "src/main.c")?;
```

This gives you:

- `parsed.source`: the preprocessed source text
- `parsed.unit`: the AST root

## Parse preprocessed text

Use this when another tool already ran preprocessing and you only want PAC to parse.

```rust
use parc::driver::{parse_preprocessed, Config};

let config = Config::default();
let source = r#"
# 1 "generated.i"
typedef int count_t;
count_t answer(void) { return 42; }
"#
.to_string();

let parsed = parse_preprocessed(&config, source)?;
```

This is useful for:

- snapshot-based tests
- integration with custom build systems
- reproducing parse bugs from stored `.i` files

## Parse a fragment

Use `parc::parse` when you are not dealing with a whole file.

```rust
use parc::driver::Flavor;
use parc::parse;

let expr = parse::expression("ptr->len + 1", Flavor::GnuC11)?;
let decl = parse::declaration("unsigned long flags;", Flavor::StdC11)?;
let stmt = parse::statement("if (ok) return 1;", Flavor::StdC11)?;
```

This is the right choice for:

- unit tests
- parser experiments
- editor tooling for partial snippets

## Build an analyzer

The normal analyzer flow is:

1. Parse with `driver` or `parse`
2. Traverse with `visit`
3. Use `span` and `loc` for diagnostics

Example outline:

```rust
use parc::driver::{parse, Config};
use parc::visit::{self, Visit};
use parc::{ast, span};

struct FunctionCounter {
    count: usize,
}

impl<'ast> Visit<'ast> for FunctionCounter {
    fn visit_function_definition(
        &mut self,
        node: &'ast ast::FunctionDefinition,
        span: &'ast span::Span,
    ) {
        self.count += 1;
        visit::visit_function_definition(self, node, span);
    }
}

let parsed = parse(&Config::default(), "src/main.c")?;
let mut counter = FunctionCounter { count: 0 };
counter.visit_translation_unit(&parsed.unit);
println!("functions: {}", counter.count);
# Ok::<(), parc::driver::Error>(())
```

## Debug the parse tree

Use the printer when you need a human-readable structural dump:

```rust
use parc::driver::{parse, Config};
use parc::print::Printer;
use parc::visit::Visit;

let parsed = parse(&Config::default(), "src/main.c")?;

let mut out = String::new();
Printer::new(&mut out).visit_translation_unit(&parsed.unit);
println!("{}", out);
# Ok::<(), parc::driver::Error>(())
```

## Rule of thumb

- If preprocessing matters, start with `driver`.
- If you already have plain text in memory, start with `parse`.
- If you need diagnostics tied back to original files, keep the preprocessed source string.
