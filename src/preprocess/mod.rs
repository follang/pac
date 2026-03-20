//! Built-in C preprocessor
//!
//! Provides tokenization, directive parsing, macro expansion, and
//! conditional compilation without requiring an external `gcc -E` invocation.

mod token;
mod lexer;

pub use self::token::{Token, TokenKind};
pub use self::lexer::Lexer;
