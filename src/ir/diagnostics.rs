//! Source-level diagnostics for the PARC frontend contract.
//!
//! These diagnostics cover preprocessing and parsing concerns only.
//! Link/binary/probe diagnostics belong to downstream crates.

use serde::{Deserialize, Serialize};

/// Severity of a frontend diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Warning,
    Error,
}

/// Classification of frontend diagnostic kinds.
///
/// Only source-level concerns are represented here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticKind {
    PreprocessingFailed,
    ParseFailed,
    DeclarationUnsupported,
    DeclarationPartial,
}

/// Location within a source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: Option<String>,
    pub offset: usize,
    #[serde(default)]
    pub line: Option<usize>,
    #[serde(default)]
    pub column: Option<usize>,
}

/// One frontend diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDiagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub message: String,
    #[serde(default)]
    pub location: Option<SourceLocation>,
    #[serde(default)]
    pub item_name: Option<String>,
}

impl SourceDiagnostic {
    pub fn error(kind: DiagnosticKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            severity: Severity::Error,
            message: message.into(),
            location: None,
            item_name: None,
        }
    }

    pub fn warning(kind: DiagnosticKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            severity: Severity::Warning,
            message: message.into(),
            location: None,
            item_name: None,
        }
    }

    pub fn with_location(mut self, file: Option<String>, offset: usize) -> Self {
        self.location = Some(SourceLocation {
            file,
            offset,
            line: None,
            column: None,
        });
        self
    }

    pub fn with_item(mut self, name: impl Into<String>) -> Self {
        self.item_name = Some(name.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_construction() {
        let d = SourceDiagnostic::error(DiagnosticKind::ParseFailed, "unexpected token");
        assert_eq!(d.severity, Severity::Error);
        assert_eq!(d.kind, DiagnosticKind::ParseFailed);
        assert!(d.location.is_none());
        assert!(d.item_name.is_none());
    }

    #[test]
    fn warning_with_location_and_item() {
        let d = SourceDiagnostic::warning(DiagnosticKind::DeclarationPartial, "bitfield ignored")
            .with_location(Some("test.h".into()), 42)
            .with_item("my_struct");
        assert_eq!(d.severity, Severity::Warning);
        assert_eq!(d.location.as_ref().unwrap().offset, 42);
        assert_eq!(d.item_name.as_deref(), Some("my_struct"));
    }

    #[test]
    fn json_roundtrip() {
        let d = SourceDiagnostic::error(DiagnosticKind::DeclarationUnsupported, "cannot model")
            .with_item("complex_union")
            .with_location(Some("api.h".into()), 100);
        let json = serde_json::to_string(&d).unwrap();
        let back: SourceDiagnostic = serde_json::from_str(&json).unwrap();
        assert_eq!(d, back);
    }

    #[test]
    fn all_diagnostic_kinds() {
        let kinds = vec![
            DiagnosticKind::PreprocessingFailed,
            DiagnosticKind::ParseFailed,
            DiagnosticKind::DeclarationUnsupported,
            DiagnosticKind::DeclarationPartial,
        ];
        for kind in kinds {
            let d = SourceDiagnostic::error(kind.clone(), "test");
            assert_eq!(d.kind, kind);
        }
    }
}
