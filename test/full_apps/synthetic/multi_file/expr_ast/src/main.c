#include "expr_api.h"

int expr_eval(const ExprNode *node)
{
    if (node->tag == EXPR_VALUE) {
        return node->value;
    }
    return expr_eval(node->left) + expr_eval(node->right);
}

int main(void)
{
    ExprNode leaf = { EXPR_VALUE, 3, (const ExprNode *)0, (const ExprNode *)0 };
    ExprNode root = { EXPR_SUM, 0, &leaf, &leaf };
    return expr_eval(&root) == 6 ? 0 : 1;
}
