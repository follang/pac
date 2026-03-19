#ifndef EXPR_NODES_H
#define EXPR_NODES_H

typedef enum ExprTag {
    EXPR_VALUE,
    EXPR_SUM
} ExprTag;

typedef struct ExprNode {
    ExprTag tag;
    int value;
    const struct ExprNode *left;
    const struct ExprNode *right;
} ExprNode;

#endif
