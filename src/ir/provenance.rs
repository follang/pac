//! Source provenance tracking for the PARC frontend contract.

use serde::{Deserialize, Serialize};

use super::diagnostics::SourceLocation;
use super::items::SourceItemKind;

/// Classification of where a declaration originated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceOrigin {
    /// From one of the user's entry headers.
    Entry,
    /// Included from entry headers but not a system header.
    UserInclude,
    /// System header.
    System,
    /// Origin could not be determined.
    Unknown,
}

/// Per-declaration provenance metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DeclarationProvenance {
    #[serde(default)]
    pub item_name: Option<String>,
    #[serde(default)]
    pub item_kind: Option<SourceItemKind>,
    #[serde(default)]
    pub source_offset: Option<usize>,
    #[serde(default)]
    pub source_origin: Option<SourceOrigin>,
    #[serde(default)]
    pub source_location: Option<SourceLocation>,
}

/// Per-macro provenance metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MacroProvenance {
    pub macro_name: String,
    #[serde(default)]
    pub source_origin: Option<SourceOrigin>,
    #[serde(default)]
    pub source_location: Option<SourceLocation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declaration_provenance_default() {
        let p = DeclarationProvenance::default();
        assert!(p.item_name.is_none());
        assert!(p.item_kind.is_none());
        assert!(p.source_offset.is_none());
        assert!(p.source_origin.is_none());
        assert!(p.source_location.is_none());
    }

    #[test]
    fn declaration_provenance_populated() {
        let p = DeclarationProvenance {
            item_name: Some("malloc".into()),
            item_kind: Some(SourceItemKind::Function),
            source_offset: Some(42),
            source_origin: Some(SourceOrigin::Entry),
            source_location: Some(SourceLocation {
                file: Some("stdlib.h".into()),
                offset: 42,
                line: Some(10),
                column: Some(1),
            }),
        };
        assert_eq!(p.item_name.as_deref(), Some("malloc"));
        assert_eq!(p.source_origin, Some(SourceOrigin::Entry));
    }

    #[test]
    fn macro_provenance() {
        let p = MacroProvenance {
            macro_name: "SIZE_MAX".into(),
            source_origin: Some(SourceOrigin::System),
            source_location: None,
        };
        assert_eq!(p.macro_name, "SIZE_MAX");
        assert_eq!(p.source_origin, Some(SourceOrigin::System));
    }

    #[test]
    fn json_roundtrip() {
        let p = DeclarationProvenance {
            item_name: Some("point".into()),
            item_kind: Some(SourceItemKind::Record),
            source_offset: Some(100),
            source_origin: Some(SourceOrigin::UserInclude),
            source_location: Some(SourceLocation {
                file: Some("types.h".into()),
                offset: 100,
                line: Some(5),
                column: Some(8),
            }),
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: DeclarationProvenance = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn all_origins() {
        let origins = vec![
            SourceOrigin::Entry,
            SourceOrigin::UserInclude,
            SourceOrigin::System,
            SourceOrigin::Unknown,
        ];
        for o in &origins {
            let json = serde_json::to_string(o).unwrap();
            let back: SourceOrigin = serde_json::from_str(&json).unwrap();
            assert_eq!(*o, back);
        }
    }
}
