//! Differential migration tests: pac extraction vs expected bic extraction behavior.
//!
//! These tests verify that pac extraction produces semantically equivalent output
//! to what bic's extractor would produce, documenting intentional differences.
//!
//! ## Known intentional differences
//!
//! 1. **Pointer model**: bic uses `const_pointee: bool` + `qualifiers` on Pointer;
//!    pac uses only `qualifiers` on Pointer where `qualifiers.is_const` means the
//!    pointee is const-qualified. This is semantically equivalent but structurally
//!    different in serialized form.
//!
//! 2. **No FieldLayout**: bic's `FieldBinding` has `layout: Option<FieldLayout>`;
//!    pac's `SourceField` does not carry layout because ABI measurement belongs to LINC.
//!
//! 3. **No RecordRepresentation**: bic's `RecordBinding` has `representation` and
//!    `abi_confidence`; pac does not carry these because ABI proof belongs to LINC.
//!
//! 4. **No canonical_resolution on TypeAlias**: bic resolves typedef chains eagerly;
//!    pac preserves TypedefRef chains as-is.
//!
//! 5. **Naming**: SourceType vs BindingType, SourceItem vs BindingItem, etc.
//!
//! 6. **CallingConvention**: pac does not have `Unknown(String)` variant (uses C as default).

use crate::extract;
use crate::ir::*;

fn extract_ok(src: &str) -> SourcePackage {
    extract::extract_from_source(src).expect("parse failed")
}

// --- Simple declarations: equivalent behavior ---

#[test]
fn diff_typedef_int() {
    // bic: TypeAliasBinding { name: "my_int", target: BindingType::Int, ... }
    // pac: SourceTypeAlias { name: "my_int", target: SourceType::Int, ... }
    let pkg = extract_ok("typedef int my_int;");
    let alias = pkg.find_type_alias("my_int").unwrap();
    assert_eq!(alias.name, "my_int");
    assert_eq!(alias.target, SourceType::Int);
    assert!(alias.source_offset.is_some());
}

#[test]
fn diff_typedef_pointer() {
    // bic: Pointer { pointee: Void, const_pointee: false, qualifiers: default }
    // pac: Pointer { pointee: Void, qualifiers: { is_const: false, ... } }
    // Difference: bic has explicit const_pointee field; pac uses qualifiers.is_const
    let pkg = extract_ok("typedef void *handle_t;");
    let alias = pkg.find_type_alias("handle_t").unwrap();
    assert_eq!(alias.target, SourceType::ptr(SourceType::Void));
}

#[test]
fn diff_typedef_const_pointer() {
    // bic: Pointer { pointee: Char, const_pointee: true, qualifiers: default }
    // pac: Pointer { pointee: Char, qualifiers: { is_const: true, ... } }
    // Both represent "const char *" — the const applies to pointee
    let pkg = extract_ok("typedef const char *cstr_t;");
    let alias = pkg.find_type_alias("cstr_t").unwrap();
    assert_eq!(alias.target, SourceType::const_ptr(SourceType::Char));
}

#[test]
fn diff_function_basic() {
    // Both extractors: name, return type, parameters, variadic, calling convention
    let pkg = extract_ok("int puts(const char *s);");
    let f = pkg.find_function("puts").unwrap();
    assert_eq!(f.name, "puts");
    assert_eq!(f.return_type, SourceType::Int);
    assert!(!f.variadic);
    assert_eq!(f.calling_convention, CallingConvention::C);
    assert_eq!(f.parameters.len(), 1);
    assert_eq!(f.parameters[0].name.as_deref(), Some("s"));
    assert_eq!(f.parameters[0].ty, SourceType::const_ptr(SourceType::Char));
}

#[test]
fn diff_function_variadic() {
    // Equivalent behavior in both extractors
    let pkg = extract_ok("int printf(const char *fmt, ...);");
    let f = pkg.find_function("printf").unwrap();
    assert!(f.variadic);
    assert_eq!(f.parameters.len(), 1);
}

#[test]
fn diff_function_void_param() {
    // Both extractors collapse (void) to empty params
    let pkg = extract_ok("void abort(void);");
    let f = pkg.find_function("abort").unwrap();
    assert!(f.parameters.is_empty());
    assert_eq!(f.return_type, SourceType::Void);
}

#[test]
fn diff_struct_complete() {
    // bic: RecordBinding { kind: Struct, name: Some("point"), fields: Some([...]),
    //       representation: None, abi_confidence: None }
    // pac: SourceRecord { kind: Struct, name: Some("point"), fields: Some([...]) }
    // Difference: pac has no representation or abi_confidence
    let pkg = extract_ok("struct point { int x; int y; };");
    let rec = pkg.find_record("point").unwrap();
    assert_eq!(rec.kind, RecordKind::Struct);
    assert!(!rec.is_opaque());
    let fields = rec.fields.as_ref().unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name.as_deref(), Some("x"));
    assert_eq!(fields[0].ty, SourceType::Int);
}

#[test]
fn diff_struct_opaque() {
    // Both extractors: fields == None means opaque
    let pkg = extract_ok("struct FILE;");
    let rec = pkg.find_record("FILE").unwrap();
    assert!(rec.is_opaque());
}

#[test]
fn diff_union() {
    // Equivalent behavior
    let pkg = extract_ok("union data { int i; float f; };");
    let rec = pkg.find_record("data").unwrap();
    assert_eq!(rec.kind, RecordKind::Union);
}

#[test]
fn diff_enum_explicit_values() {
    // Equivalent behavior for explicit values
    let pkg = extract_ok("enum color { RED = 0, GREEN = 1, BLUE = 2 };");
    let e = pkg.find_enum("color").unwrap();
    assert_eq!(e.variants.len(), 3);
    assert_eq!(e.variants[0].value, Some(0));
    assert_eq!(e.variants[1].value, Some(1));
    assert_eq!(e.variants[2].value, Some(2));
}

#[test]
fn diff_enum_implicit_values() {
    // bic: implicit enum values are None
    // pac: same — implicit values are None (let the consumer number them)
    let pkg = extract_ok("enum dir { UP, DOWN, LEFT, RIGHT };");
    let e = pkg.find_enum("dir").unwrap();
    assert_eq!(e.variants[0].value, None);
    assert_eq!(e.variants[1].value, None);
}

#[test]
fn diff_extern_variable() {
    // Equivalent behavior
    let pkg = extract_ok("extern int errno;");
    let v = pkg.find_variable("errno").unwrap();
    assert_eq!(v.name, "errno");
    assert_eq!(v.ty, SourceType::Int);
}

#[test]
fn diff_static_function_skipped() {
    // Both extractors skip static functions and emit diagnostics
    let pkg = extract_ok("static int helper(void) { return 0; }");
    assert_eq!(pkg.function_count(), 0);
    assert!(pkg.has_diagnostics());
}

// --- Hostile declarations: documenting behavior ---

#[test]
fn diff_typedef_chain() {
    // bic: has canonical_resolution field that eagerly resolves chains
    // pac: preserves TypedefRef at each level (no eager resolution)
    // This is an INTENTIONAL difference — pac is more faithful to source
    let src = r#"
typedef unsigned int uint32_t;
typedef uint32_t my_uint;
"#;
    let pkg = extract_ok(src);
    let my_uint = pkg.find_type_alias("my_uint").unwrap();
    // pac preserves the chain — target is TypedefRef, not eagerly resolved UInt
    assert_eq!(
        my_uint.target,
        SourceType::TypedefRef("uint32_t".into())
    );
}

#[test]
fn diff_bitfield() {
    // bic: FieldBinding { ..., bit_width: Some(3), layout: None }
    // pac: SourceField { ..., bit_width: Some(3) } — no layout field at all
    // Both emit diagnostics about bitfield limitations
    let pkg = extract_ok("struct flags { unsigned a:3; };");
    let rec = pkg.records().next().unwrap();
    let fields = rec.fields.as_ref().unwrap();
    assert_eq!(fields[0].bit_width, Some(3));
    assert!(pkg.has_diagnostics());
}

#[test]
fn diff_function_definition_extracted_as_declaration() {
    // Both extractors treat function definitions as declarations-with-body-ignored
    let pkg = extract_ok("int compute(int x) { return x * 2; }");
    let f = pkg.find_function("compute").unwrap();
    assert_eq!(f.name, "compute");
    assert_eq!(f.return_type, SourceType::Int);
    assert_eq!(f.parameters.len(), 1);
}

#[test]
fn diff_typedef_struct_inline() {
    // Both extractors produce both the inline struct and the typedef
    let pkg = extract_ok("typedef struct point { int x; int y; } point_t;");
    assert!(pkg.find_record("point").is_some());
    assert!(pkg.find_type_alias("point_t").is_some());
    let alias = pkg.find_type_alias("point_t").unwrap();
    assert_eq!(alias.target, SourceType::RecordRef("point".into()));
}

#[test]
fn diff_function_pointer_typedef() {
    // Both extractors produce Pointer { pointee: FunctionPointer { ... } }
    let pkg = extract_ok("typedef void (*handler_t)(int sig);");
    let alias = pkg.find_type_alias("handler_t").unwrap();
    match &alias.target {
        SourceType::Pointer { pointee, .. } => match pointee.as_ref() {
            SourceType::FunctionPointer { return_type, parameters, variadic } => {
                assert_eq!(**return_type, SourceType::Void);
                assert_eq!(parameters.len(), 1);
                assert!(!variadic);
            }
            other => panic!("expected FunctionPointer, got {:?}", other),
        },
        other => panic!("expected Pointer, got {:?}", other),
    }
}

// --- Package-level structure ---

#[test]
fn diff_package_structure() {
    // bic: BindingPackage has layouts, link, effective_macro_environment, bic_version
    // pac: SourcePackage does NOT have these (they belong to LINC)
    // pac: has schema_version, target, inputs, items, macros, diagnostics, provenance
    let pkg = extract_ok("int x(void);");

    // These fields exist on SourcePackage
    let _schema = pkg.schema_version;
    let _items = &pkg.items;
    let _diags = &pkg.diagnostics;
    let _macros = &pkg.macros;
    let _target = &pkg.target;
    let _inputs = &pkg.inputs;
    let _prov = &pkg.provenance;

    // These do NOT exist on SourcePackage (intentionally):
    // pkg.layouts — ABI measurement
    // pkg.link — link surface
    // pkg.bic_version — bic-specific
}

#[test]
fn diff_item_ordering_deterministic() {
    // Both extractors preserve declaration order
    let src = r#"
typedef int a_t;
struct b_s { int x; };
void c_fn(void);
extern int d_var;
enum e_e { E1 };
"#;
    let pkg1 = extract_ok(src);
    let pkg2 = extract_ok(src);
    let json1 = serde_json::to_string(&pkg1).unwrap();
    let json2 = serde_json::to_string(&pkg2).unwrap();
    assert_eq!(json1, json2);

    // Verify order matches source order
    let names: Vec<_> = pkg1.items.iter().map(|i| i.name().unwrap_or("?")).collect();
    assert_eq!(names[0], "a_t");
    assert_eq!(names[1], "b_s");
    assert_eq!(names[2], "c_fn");
    assert_eq!(names[3], "d_var");
    assert_eq!(names[4], "e_e");
}

// --- Real-header-style fixtures ---

#[test]
fn diff_real_header_stdlib_style() {
    let src = r#"
typedef unsigned long size_t;
typedef long ssize_t;
typedef int pid_t;
typedef unsigned int uid_t;

void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void abort(void);
void exit(int status);
int atexit(void (*function)(void));
char *getenv(const char *name);
int system(const char *command);

int atoi(const char *nptr);
long atol(const char *nptr);
"#;
    let pkg = extract_ok(src);

    // Type aliases
    assert_eq!(pkg.type_alias_count(), 4);

    // Functions (malloc, calloc, realloc, free, abort, exit, atexit, getenv, system, atoi, atol)
    assert_eq!(pkg.function_count(), 11);
    let malloc = pkg.find_function("malloc").unwrap();
    assert_eq!(malloc.return_type, SourceType::ptr(SourceType::Void));

    let atexit = pkg.find_function("atexit").unwrap();
    assert_eq!(atexit.parameters.len(), 1);
    // atexit's param is a function pointer
    match &atexit.parameters[0].ty {
        SourceType::Pointer { pointee, .. } => match pointee.as_ref() {
            SourceType::FunctionPointer { .. } => {}
            other => panic!("expected FunctionPointer, got {:?}", other),
        },
        other => panic!("expected Pointer, got {:?}", other),
    }

    // JSON roundtrip matches bic's contract
    let json = serde_json::to_string_pretty(&pkg).unwrap();
    let back: SourcePackage = serde_json::from_str(&json).unwrap();
    assert_eq!(pkg, back);
}

#[test]
fn diff_real_header_posix_io_style() {
    let src = r#"
typedef unsigned long size_t;
typedef long ssize_t;
typedef int mode_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;

struct stat;

int open(const char *pathname, int flags, ...);
int close(int fd);
ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
int fstat(int fd, struct stat *statbuf);
int chmod(const char *pathname, mode_t mode);
int chown(const char *pathname, uid_t owner, gid_t group);
"#;
    let pkg = extract_ok(src);

    assert!(pkg.find_record("stat").unwrap().is_opaque());
    assert!(pkg.find_function("open").unwrap().variadic);
    assert_eq!(pkg.find_function("read").unwrap().parameters.len(), 3);

    let write_fn = pkg.find_function("write").unwrap();
    assert_eq!(
        write_fn.parameters[1].ty,
        SourceType::const_ptr(SourceType::Void)
    );
}
