enum ExprKind {
    EXPR_CONST,
    EXPR_ADD,
    EXPR_NEG
};

struct Expr {
    enum ExprKind kind;
    int value;
    const struct Expr *left;
    const struct Expr *right;
};

static int eval_expr(const struct Expr *expr)
{
    switch (expr->kind) {
    case EXPR_CONST:
        return expr->value;
    case EXPR_ADD:
        return eval_expr(expr->left) + eval_expr(expr->right);
    case EXPR_NEG:
        return -eval_expr(expr->left);
    }
    return 0;
}

static struct Expr literals[] = {
    { EXPR_CONST, 4, (const struct Expr *)0, (const struct Expr *)0 },
    { EXPR_CONST, 9, (const struct Expr *)0, (const struct Expr *)0 }
};
