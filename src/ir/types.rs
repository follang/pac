//! Source-level type representation for the PARC frontend contract.
//!
//! These types model C declarations as seen in source, not as proven by ABI probing.

use serde::{Deserialize, Serialize};

/// Source-level type representation.
///
/// This is smaller and more canonical than the parser AST type nodes.
/// It does not claim ABI layout — only source-level meaning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SourceType {
    Void,
    Bool,
    Char,
    SChar,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,
    ULongLong,
    Float,
    Double,
    LongDouble,
    Int128,
    UInt128,
    Pointer {
        pointee: Box<SourceType>,
        #[serde(default)]
        qualifiers: TypeQualifiers,
    },
    Array(Box<SourceType>, Option<u64>),
    Qualified {
        ty: Box<SourceType>,
        #[serde(default)]
        qualifiers: TypeQualifiers,
    },
    FunctionPointer {
        return_type: Box<SourceType>,
        parameters: Vec<SourceType>,
        variadic: bool,
    },
    TypedefRef(String),
    RecordRef(String),
    EnumRef(String),
    /// A type that the extractor could not resolve.
    Opaque(String),
}

/// Type qualifiers attached to a declaration or sub-type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TypeQualifiers {
    #[serde(default)]
    pub is_const: bool,
    #[serde(default)]
    pub is_volatile: bool,
    #[serde(default)]
    pub is_restrict: bool,
    #[serde(default)]
    pub is_atomic: bool,
}

impl SourceType {
    /// Create a pointer to the given type with default qualifiers.
    pub fn ptr(pointee: SourceType) -> Self {
        SourceType::Pointer {
            pointee: Box::new(pointee),
            qualifiers: TypeQualifiers::default(),
        }
    }

    /// Create a const-qualified pointer to the given type.
    pub fn const_ptr(pointee: SourceType) -> Self {
        SourceType::Pointer {
            pointee: Box::new(pointee),
            qualifiers: TypeQualifiers {
                is_const: true,
                ..Default::default()
            },
        }
    }

    /// Wrap a type with qualifiers, collapsing no-op qualification.
    pub fn qualified(ty: SourceType, qualifiers: TypeQualifiers) -> Self {
        if qualifiers == TypeQualifiers::default() {
            ty
        } else {
            SourceType::Qualified {
                ty: Box::new(ty),
                qualifiers,
            }
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            SourceType::Void => true,
            SourceType::Qualified { ty, .. } => ty.is_void(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_types_exist() {
        let types = vec![
            SourceType::Void,
            SourceType::Bool,
            SourceType::Char,
            SourceType::SChar,
            SourceType::UChar,
            SourceType::Short,
            SourceType::UShort,
            SourceType::Int,
            SourceType::UInt,
            SourceType::Long,
            SourceType::ULong,
            SourceType::LongLong,
            SourceType::ULongLong,
            SourceType::Float,
            SourceType::Double,
            SourceType::LongDouble,
            SourceType::Int128,
            SourceType::UInt128,
        ];
        assert_eq!(types.len(), 18);
    }

    #[test]
    fn pointer_construction() {
        let p = SourceType::ptr(SourceType::Int);
        match &p {
            SourceType::Pointer { pointee, qualifiers } => {
                assert_eq!(**pointee, SourceType::Int);
                assert_eq!(*qualifiers, TypeQualifiers::default());
            }
            _ => panic!("expected pointer"),
        }
    }

    #[test]
    fn const_pointer_construction() {
        let p = SourceType::const_ptr(SourceType::Char);
        match &p {
            SourceType::Pointer { pointee, qualifiers } => {
                assert_eq!(**pointee, SourceType::Char);
                assert!(qualifiers.is_const);
            }
            _ => panic!("expected pointer"),
        }
    }

    #[test]
    fn qualified_collapses_default() {
        let t = SourceType::qualified(SourceType::Int, TypeQualifiers::default());
        assert_eq!(t, SourceType::Int);
    }

    #[test]
    fn qualified_wraps_nondefault() {
        let q = TypeQualifiers {
            is_const: true,
            ..Default::default()
        };
        let t = SourceType::qualified(SourceType::Int, q);
        match &t {
            SourceType::Qualified { ty, qualifiers } => {
                assert_eq!(**ty, SourceType::Int);
                assert!(qualifiers.is_const);
            }
            _ => panic!("expected qualified"),
        }
    }

    #[test]
    fn void_detection() {
        assert!(SourceType::Void.is_void());
        assert!(!SourceType::Int.is_void());
        let q = SourceType::Qualified {
            ty: Box::new(SourceType::Void),
            qualifiers: TypeQualifiers {
                is_const: true,
                ..Default::default()
            },
        };
        assert!(q.is_void());
    }

    #[test]
    fn array_type() {
        let a = SourceType::Array(Box::new(SourceType::Int), Some(10));
        match &a {
            SourceType::Array(elem, size) => {
                assert_eq!(**elem, SourceType::Int);
                assert_eq!(*size, Some(10));
            }
            _ => panic!("expected array"),
        }
    }

    #[test]
    fn function_pointer_type() {
        let fp = SourceType::FunctionPointer {
            return_type: Box::new(SourceType::Int),
            parameters: vec![SourceType::ptr(SourceType::Void)],
            variadic: true,
        };
        match &fp {
            SourceType::FunctionPointer {
                return_type,
                parameters,
                variadic,
            } => {
                assert_eq!(**return_type, SourceType::Int);
                assert_eq!(parameters.len(), 1);
                assert!(*variadic);
            }
            _ => panic!("expected function pointer"),
        }
    }

    #[test]
    fn ref_types() {
        let _ = SourceType::TypedefRef("size_t".into());
        let _ = SourceType::RecordRef("point".into());
        let _ = SourceType::EnumRef("color".into());
        let _ = SourceType::Opaque("__builtin_va_list".into());
    }

    #[test]
    fn json_roundtrip() {
        let types = vec![
            SourceType::Void,
            SourceType::ptr(SourceType::Char),
            SourceType::const_ptr(SourceType::Void),
            SourceType::Array(Box::new(SourceType::Int), Some(5)),
            SourceType::FunctionPointer {
                return_type: Box::new(SourceType::Int),
                parameters: vec![SourceType::Long, SourceType::ptr(SourceType::Char)],
                variadic: false,
            },
            SourceType::TypedefRef("uint32_t".into()),
            SourceType::RecordRef("stat".into()),
            SourceType::EnumRef("mode".into()),
            SourceType::Opaque("complex double".into()),
        ];
        for ty in &types {
            let json = serde_json::to_string(ty).unwrap();
            let back: SourceType = serde_json::from_str(&json).unwrap();
            assert_eq!(*ty, back, "roundtrip failed for {:?}", ty);
        }
    }

    #[test]
    fn deterministic_serialization() {
        let ty = SourceType::Pointer {
            pointee: Box::new(SourceType::Int),
            qualifiers: TypeQualifiers {
                is_const: true,
                is_volatile: false,
                is_restrict: false,
                is_atomic: false,
            },
        };
        let a = serde_json::to_string(&ty).unwrap();
        let b = serde_json::to_string(&ty).unwrap();
        assert_eq!(a, b);
    }
}
