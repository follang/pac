//! Built-in C preprocessor
//!
//! Provides tokenization, directive parsing, macro expansion, and
//! conditional compilation without requiring an external `gcc -E` invocation.

mod token;
mod lexer;
mod directive;
mod macros;
mod expr;
mod processor;
mod include;
mod predefined;

pub use self::token::{Token, TokenKind};
pub use self::lexer::Lexer;
pub use self::directive::{Directive, parse_directive};
pub use self::macros::{MacroDef, MacroTable};
pub use self::expr::eval_condition;
pub use self::processor::{Processor, ProcessorOutput, IncludeRequest, preprocess, tokens_to_text};
pub use self::include::{IncludeResolver, PreprocessResult};
pub use self::predefined::{Target, TargetArch, TargetOs, define_target_macros};
