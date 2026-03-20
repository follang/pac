/// A preprocessor token with its source position.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub offset: usize,
}

/// Token classification for the preprocessor.
///
/// This is simpler than a full C lexer — the preprocessor only needs to
/// distinguish identifiers, numbers, strings, punctuation, whitespace,
/// newlines, and comments. It does not need to resolve types or parse
/// expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// An identifier or keyword: `FOO`, `int`, `__GNUC__`
    Ident,
    /// A preprocessing number: `42`, `0xFF`, `3.14`, `1e10`
    /// (superset of integer and float constants per §6.4.8)
    Number,
    /// A string literal including quotes: `"hello"`
    StringLiteral,
    /// A character constant including quotes: `'x'`
    CharLiteral,
    /// A `#` at the start of a line (directive introducer)
    Hash,
    /// A `##` token-paste operator
    HashHash,
    /// One or more whitespace characters (not newlines)
    Whitespace,
    /// A newline character (significant for directive termination)
    Newline,
    /// A single-line `//` comment (treated as whitespace)
    LineComment,
    /// A block `/* ... */` comment (treated as whitespace)
    BlockComment,
    /// Single punctuation character: `(`, `)`, `,`, `;`, `{`, `}`, etc.
    Punct,
    /// End of input
    Eof,
    /// Anything that doesn't fit above
    Other,
}
