//! Source-level macro representation for the PARC frontend contract.

use serde::{Deserialize, Serialize};

/// High-level interpretation of a macro body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroKind {
    Integer,
    String,
    Expression,
    Other,
}

/// Whether a macro is object-like or function-like.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MacroForm {
    #[default]
    ObjectLike,
    FunctionLike,
}

/// Parsed constant value for macros that can be lowered directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroValue {
    Integer(i128),
    String(String),
}

/// One captured preprocessor macro in the source contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMacro {
    pub name: String,
    pub body: String,
    pub form: MacroForm,
    pub kind: MacroKind,
    #[serde(default)]
    pub value: Option<MacroValue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_like_integer_macro() {
        let m = SourceMacro {
            name: "API_LEVEL".into(),
            body: "7".into(),
            form: MacroForm::ObjectLike,
            kind: MacroKind::Integer,
            value: Some(MacroValue::Integer(7)),
        };
        assert_eq!(m.form, MacroForm::ObjectLike);
        assert_eq!(m.value, Some(MacroValue::Integer(7)));
    }

    #[test]
    fn function_like_macro() {
        let m = SourceMacro {
            name: "MAX".into(),
            body: "((a) > (b) ? (a) : (b))".into(),
            form: MacroForm::FunctionLike,
            kind: MacroKind::Expression,
            value: None,
        };
        assert_eq!(m.form, MacroForm::FunctionLike);
        assert!(m.value.is_none());
    }

    #[test]
    fn string_macro() {
        let m = SourceMacro {
            name: "VERSION".into(),
            body: "\"1.0\"".into(),
            form: MacroForm::ObjectLike,
            kind: MacroKind::String,
            value: Some(MacroValue::String("1.0".into())),
        };
        assert_eq!(m.value, Some(MacroValue::String("1.0".into())));
    }

    #[test]
    fn json_roundtrip() {
        let macros = vec![
            SourceMacro {
                name: "SIZE".into(),
                body: "42".into(),
                form: MacroForm::ObjectLike,
                kind: MacroKind::Integer,
                value: Some(MacroValue::Integer(42)),
            },
            SourceMacro {
                name: "MIN".into(),
                body: "((a)<(b)?(a):(b))".into(),
                form: MacroForm::FunctionLike,
                kind: MacroKind::Expression,
                value: None,
            },
        ];
        for m in &macros {
            let json = serde_json::to_string(m).unwrap();
            let back: SourceMacro = serde_json::from_str(&json).unwrap();
            assert_eq!(*m, back);
        }
    }

    #[test]
    fn default_form_is_object_like() {
        assert_eq!(MacroForm::default(), MacroForm::ObjectLike);
    }
}
