# Driver API

The `driver` module is the high-level API for file parsing. It runs a system preprocessor, then
parses the resulting text into a `TranslationUnit`.

## Main types

```rust
pub struct Config {
    pub cpp_command: String,
    pub cpp_options: Vec<String>,
    pub flavor: Flavor,
}

pub enum Flavor {
    StdC11,
    GnuC11,
    ClangC11,
}

pub struct Parse {
    pub source: String,
    pub unit: TranslationUnit,
}
```

The return value matters:

- `source` is the preprocessed source PAC actually parsed
- `unit` is the AST root

## Basic file parsing

```rust
use pac::driver::{parse, Config};

let config = Config::default();
let parsed = parse(&config, "examples/demo.c")?;

println!("preprocessed bytes: {}", parsed.source.len());
println!("top-level nodes: {}", parsed.unit.0.len());
# Ok::<(), pac::driver::Error>(())
```

## Configuring the preprocessor

You can override both the preprocessor executable and its arguments.

```rust
use pac::driver::{parse, Config, Flavor};

let config = Config {
    cpp_command: "gcc".into(),
    cpp_options: vec![
        "-E".into(),
        "-Iinclude".into(),
        "-DMODE=2".into(),
        "-nostdinc".into(),
    ],
    flavor: Flavor::GnuC11,
};

let parsed = parse(&config, "src/input.c")?;
# Ok::<(), pac::driver::Error>(())
```

This is the place to inject:

- include directories with `-I...`
- macro definitions with `-D...`
- stricter or more isolated builds with `-nostdinc`

## GCC vs Clang helpers

The convenience constructors also select parser flavor:

```rust
use pac::driver::Config;

let gcc = Config::with_gcc();     // gcc -E, GNU flavor
let clang = Config::with_clang(); // clang -E, Clang flavor
```

Use these when you want the parser flavor to match the syntax accepted by the external
preprocessor.

## Parsing preprocessed text directly

If you already have `.i`-style content, skip `parse` and call `parse_preprocessed`.

```rust
use pac::driver::{parse_preprocessed, Config};

let source = r#"
# 1 "sample.i"
typedef int count_t;
count_t next(count_t x) { return x + 1; }
"#
.to_string();

let parsed = parse_preprocessed(&Config::default(), source)?;
println!("{}", parsed.unit.0.len());
# Ok::<(), pac::driver::SyntaxError>(())
```

## Error model

`driver::parse` returns:

```rust
Result<Parse, pac::driver::Error>
```

The error variants are:

- `PreprocessorError(io::Error)` when the external preprocessor fails
- `SyntaxError(SyntaxError)` when preprocessing succeeded but parsing failed

## Working with syntax errors

`SyntaxError` includes:

- `source`: the preprocessed source
- `line`, `column`, `offset`: the parse failure position in that source
- `expected`: a set of expected tokens

Example:

```rust
use pac::driver::{parse_preprocessed, Config};

let broken = "int main( { return 0; }".to_string();
match parse_preprocessed(&Config::default(), broken) {
    Ok(_) => {}
    Err(err) => {
        eprintln!("parse failed at {}:{}", err.line, err.column);
        eprintln!("expected: {:?}", err.expected);
    }
}
```

If the preprocessed source contains line markers, `SyntaxError::get_location()` can reconstruct the
original file and include stack.

## Practical advice

- Keep `parsed.source` if you plan to report errors later.
- Use `parse_preprocessed` for deterministic regression tests.
- Prefer explicit `cpp_options` in tools and CI so parse behavior stays reproducible.
