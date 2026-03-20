# Migration From bic

This chapter documents how to migrate downstream consumers from `bic`'s frontend extraction
to `parc`'s `SourcePackage` contract.

## Why migrate

`parc` now owns source-level declaration extraction. `bic`'s `extract.rs` was the legacy location
for this logic. The canonical path is now:

```text
C headers  ->  parc::scan / parc::extract  ->  SourcePackage  ->  downstream
```

`bic` should consume `parc::ir::SourcePackage` instead of owning its own extraction.

## Type mapping

| bic type | parc type | Notes |
| --- | --- | --- |
| `BindingPackage` | `SourcePackage` | parc has no `layouts`, `link`, or `bic_version` |
| `BindingItem` | `SourceItem` | Same variant set |
| `BindingType` | `SourceType` | Pointer model differs (see below) |
| `FunctionBinding` | `SourceFunction` | Identical structure |
| `ParameterBinding` | `SourceParameter` | Identical structure |
| `RecordBinding` | `SourceRecord` | No `representation` or `abi_confidence` |
| `FieldBinding` | `SourceField` | No `layout` field |
| `EnumBinding` | `SourceEnum` | Identical structure |
| `TypeAliasBinding` | `SourceTypeAlias` | No `canonical_resolution` |
| `VariableBinding` | `SourceVariable` | Identical structure |
| `UnsupportedItem` | `SourceUnsupported` | Identical structure |
| `CallingConvention` | `CallingConvention` | parc version includes `Unknown(String)` |
| `TypeQualifiers` | `TypeQualifiers` | Identical structure |
| `BindingTarget` | `SourceTarget` | Identical structure |
| `BindingInputs` | `SourceInputs` | Identical structure |
| `BindingDefine` | `SourceDefine` | Identical structure |
| `MacroBinding` | `SourceMacro` | parc drops `function_like` and `category` |
| `DeclarationProvenance` | `DeclarationProvenance` | Identical structure |
| `MacroProvenance` | `MacroProvenance` | Identical structure |

## Pointer model difference

bic:

```rust
Pointer {
    pointee: Box<BindingType>,
    const_pointee: bool,      // whether pointee is const
    qualifiers: TypeQualifiers, // qualifiers on the pointer itself
}
```

parc:

```rust
Pointer {
    pointee: Box<SourceType>,
    qualifiers: TypeQualifiers, // is_const means pointee is const
}
```

In parc, `qualifiers.is_const` on a `Pointer` indicates that the pointee is const-qualified.
Use `SourceType::const_ptr(inner)` and `SourceType::ptr(inner)` as constructors.

## Missing fields in parc

These bic fields are intentionally absent from parc because they belong to the link/ABI layer:

- `FieldBinding.layout` (field offset) — use LINC probing
- `RecordBinding.representation` — use LINC probing
- `RecordBinding.abi_confidence` — use LINC validation
- `TypeAliasBinding.canonical_resolution` — parc preserves `TypedefRef` chains
- `BindingPackage.layouts` — use LINC probing
- `BindingPackage.link` — use LINC link surface
- `BindingPackage.effective_macro_environment` — use LINC macro analysis

## Migration steps

### Step 1: Replace extraction call

Before:

```rust
use bic::extract::Extractor;
use bic::ir::BindingPackage;

let extractor = Extractor::new();
let (items, diagnostics) = extractor.extract(&unit);
let mut pkg = BindingPackage::new();
pkg.items = items;
```

After:

```rust
use parc::extract;
use parc::ir::SourcePackage;

let pkg = extract::extract_from_translation_unit(&unit, Some("header.h".into()));
```

Or for end-to-end scanning:

```rust
use parc::scan::{ScanConfig, scan_headers};

let config = ScanConfig::new()
    .entry_header("header.h")
    .with_builtin_preprocessor();
let result = scan_headers(&config).unwrap();
let pkg: &SourcePackage = &result.package;
```

### Step 2: Update type references

Replace all uses of `BindingType` with `SourceType`, `BindingItem` with `SourceItem`, etc.
The variant names are identical.

### Step 3: Handle pointer model

Replace `const_pointee` checks:

```rust
// Before (bic)
if let BindingType::Pointer { const_pointee: true, .. } = ty { ... }

// After (parc)
if let SourceType::Pointer { qualifiers, .. } = ty {
    if qualifiers.is_const { ... }
}
```

### Step 4: Remove ABI fields

Any code that reads `FieldBinding.layout`, `RecordBinding.representation`, or
`RecordBinding.abi_confidence` should be moved to LINC's domain.

### Step 5: Use builder for programmatic construction

```rust
use parc::ir::{SourcePackageBuilder, SourceItem, SourceFunction, ...};

let pkg = SourcePackageBuilder::new()
    .source_path("api.h")
    .item(SourceItem::Function(func))
    .item(SourceItem::Record(rec))
    .build();
```

## API reference

Key public APIs for downstream consumers:

- `parc::extract::extract_from_source(src)` — parse and extract
- `parc::extract::extract_from_translation_unit(unit, path)` — extract from AST
- `parc::extract::parse_and_extract(src, flavor)` — with flavor control
- `parc::extract::parse_and_extract_resilient(src, flavor)` — with error recovery
- `parc::scan::scan_headers(config)` — end-to-end header scanning
- `parc::ir::SourcePackage` — the contract type
- `parc::ir::SourcePackageBuilder` — programmatic construction
- `SourcePackage::retain_items(pred)` — filter items
- `SourcePackage::merge(other)` — combine packages
