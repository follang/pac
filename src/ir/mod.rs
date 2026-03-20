//! Source-level IR for the PARC frontend contract.
//!
//! This module defines the durable intermediate representation that PARC
//! produces from parsed C source. It is:
//!
//! - Smaller and more canonical than the parser AST
//! - Serializable via serde (JSON, etc.)
//! - Independent of parser node shapes
//! - Free of link/binary/ABI-proof concerns
//!
//! Downstream consumers (LINC, GERC) should depend only on these types,
//! never on `pac::ast` directly.

pub mod items;
pub mod types;

pub use items::{
    CallingConvention, RecordKind, SourceEnum, SourceEnumVariant, SourceField, SourceFunction,
    SourceItem, SourceItemKind, SourceParameter, SourceRecord, SourceTypeAlias, SourceUnsupported,
    SourceVariable,
};
pub use types::{SourceType, TypeQualifiers};
