# Diagnostics And Printing

PAC includes two pieces that are especially useful when building tools on top of the parser:

- detailed parse errors
- a tree printer for AST inspection

## Direct parser diagnostics

The `parse` module returns `ParseError`:

```rust
use pac::driver::Flavor;
use pac::parse;

match parse::expression("a +", Flavor::StdC11) {
    Ok(_) => {}
    Err(err) => {
        eprintln!("line: {}", err.line);
        eprintln!("column: {}", err.column);
        eprintln!("offset: {}", err.offset);
        eprintln!("expected: {:?}", err.expected);
    }
}
```

This is enough for:

- editor error messages
- parser regression tests
- grammar debugging

## Driver diagnostics

The driver adds preprocessor context on top:

```rust
use pac::driver::{parse, Config, Error};

match parse(&Config::default(), "broken.c") {
    Ok(_) => {}
    Err(Error::PreprocessorError(err)) => {
        eprintln!("preprocessor failed: {}", err);
    }
    Err(Error::SyntaxError(err)) => {
        let (loc, includes) = err.get_location();
        eprintln!("syntax error in {}:{}:", loc.file, loc.line);
        eprintln!("column in preprocessed source: {}", err.column);
        for include in includes {
            eprintln!("included from {}:{}", include.file, include.line);
        }
    }
}
```

## Formatting expected tokens

`driver::SyntaxError` also has `format_expected`, which is useful when building a custom
human-readable error message.

## AST printing

`print::Printer` is a visitor that renders the tree as an indented text dump.

```rust
use pac::driver::{parse, Config};
use pac::print::Printer;
use pac::visit::Visit;

let parsed = parse(&Config::default(), "examples/sample.c")?;

let mut out = String::new();
Printer::new(&mut out).visit_translation_unit(&parsed.unit);
println!("{}", out);
# Ok::<(), pac::driver::Error>(())
```

The printer is ideal when:

- learning how PAC models a syntax form
- updating reftests
- debugging traversal code

## A practical debugging loop

When a new syntax form is not behaving the way you expect:

1. Parse the smallest reproducer with `parse::*`
2. Print the AST with `Printer`
3. Inspect spans on the nodes you care about
4. Switch to `driver` if preprocessing is involved
5. Map spans back to original files with `loc`
