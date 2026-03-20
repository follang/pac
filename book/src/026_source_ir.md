# Source IR

The `parc::ir` module defines the durable intermediate representation produced
by the PARC frontend. It is the primary contract between the parser/extractor
and downstream consumers (LINC, GERC).

## Design Principles

- **Smaller than the AST**: only normalized declarations, not the full syntax tree
- **Serializable**: all types derive `serde::Serialize` and `serde::Deserialize`
- **Parser-agnostic**: downstream consumers should depend on `parc::ir`, not `parc::ast`
- **No link/binary concerns**: no ABI probing, no library paths, no symbol validation

## Key Types

### `SourcePackage`

The top-level container:

```rust
use parc::ir::SourcePackage;

let pkg = SourcePackage::new();
assert!(pkg.is_empty());
```

### `SourceType`

Represents C types at source level:

```text
Void, Bool, Char, SChar, UChar, Short, UShort,
Int, UInt, Long, ULong, LongLong, ULongLong,
Float, Double, LongDouble, Int128, UInt128,
Pointer, Array, Qualified, FunctionPointer,
TypedefRef, RecordRef, EnumRef, Opaque
```

### `SourceItem`

One extracted declaration:

- `Function` — function declaration with name, parameters, return type, calling convention
- `Record` — struct/union with optional fields
- `Enum` — enum with named variants and optional values
- `TypeAlias` — typedef declaration
- `Variable` — extern variable declaration
- `Unsupported` — placeholder for unrepresentable declarations

### `SourceMacro`

Captured preprocessor macro with form (object-like/function-like), kind, and optional parsed value.

### `SourceDiagnostic`

Frontend diagnostic with kind, severity, message, optional location, and optional item name.

### Provenance

- `SourceOrigin` — where a declaration came from (Entry, UserInclude, System, Unknown)
- `DeclarationProvenance` — per-item provenance metadata
- `MacroProvenance` — per-macro provenance metadata
- `SourceTarget` — compiler/target identity
- `SourceInputs` — entry headers, include dirs, defines

## JSON Serialization

All IR types support JSON roundtrip:

```rust
use parc::ir::SourcePackage;

let pkg = SourcePackage::new();
let json = serde_json::to_string_pretty(&pkg).unwrap();
let back: SourcePackage = serde_json::from_str(&json).unwrap();
assert_eq!(pkg, back);
```

## Querying

`SourcePackage` provides typed accessors:

```rust
// pkg.functions()      -> Iterator<Item = &SourceFunction>
// pkg.records()        -> Iterator<Item = &SourceRecord>
// pkg.enums()          -> Iterator<Item = &SourceEnum>
// pkg.type_aliases()   -> Iterator<Item = &SourceTypeAlias>
// pkg.variables()      -> Iterator<Item = &SourceVariable>
// pkg.unsupported_items() -> ...
// pkg.find_function("malloc")
// pkg.find_record("point")
// pkg.find_enum("color")
// pkg.find_type_alias("size_t")
// pkg.find_variable("errno")
```
