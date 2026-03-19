enum TokenKind {
    TOKEN_EOF,
    TOKEN_IDENT,
    TOKEN_NUMBER,
    TOKEN_PUNCT
};

struct Token {
    enum TokenKind kind;
    const char *begin;
    unsigned length;
};

static int is_digit(char ch)
{
    return ch >= '0' && ch <= '9';
}

static struct Token scan_number(const char *text)
{
    unsigned len = 0u;
    while (is_digit(text[len])) {
        ++len;
    }
    return (struct Token){ TOKEN_NUMBER, text, len };
}
