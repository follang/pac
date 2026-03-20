# Header Scanning

The `parc::scan` module provides end-to-end header scanning: preprocess
C headers and extract a `SourcePackage` in one step.

## Quick Start

```rust
use parc::scan::{ScanConfig, scan_headers};

let config = ScanConfig::new()
    .entry_header("api.h")
    .include_dir("/usr/include")
    .define_flag("NDEBUG")
    .with_builtin_preprocessor();

let result = scan_headers(&config).unwrap();
let pkg = result.package;
```

## ScanConfig

Builder for scan configuration:

| Method | Description |
|---|---|
| `entry_header(path)` | Add an entry-point header |
| `include_dir(path)` | Add a preprocessor include search path |
| `define(name, value)` | Add a preprocessor define with value |
| `define_flag(name)` | Add a flag-style define (no value) |
| `with_compiler(cmd)` | Set the external preprocessor command |
| `with_flavor(flavor)` | Set the parser flavor |
| `with_builtin_preprocessor()` | Use the built-in preprocessor |

## Preprocessing Modes

### External (default)

Uses `gcc -E` or `clang -E` to preprocess headers. Requires the
compiler to be installed. Supports all system headers.

### Built-in

Uses `parc::preprocess` for self-contained headers that don't need
system includes. No external compiler required.

## ScanResult

The scan produces:

- `package: SourcePackage` — the extracted declarations and metadata
- `preprocessed_source: String` — the preprocessed source text

## Intake

For already-preprocessed source (e.g., output of `gcc -E`), use
`parc::intake::PreprocessedInput`:

```rust
use parc::intake::PreprocessedInput;

let input = PreprocessedInput::from_string("int foo(void);")
    .with_path("output.i")
    .with_flavor(parc::driver::Flavor::GnuC11);

let pkg = input.extract();
```
