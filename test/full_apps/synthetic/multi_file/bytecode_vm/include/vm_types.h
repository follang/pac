#ifndef VM_TYPES_H
#define VM_TYPES_H

typedef enum OpCode {
    OP_HALT,
    OP_PUSH,
    OP_ADD
} OpCode;

typedef struct VmState {
    int stack[8];
    unsigned sp;
} VmState;

#endif
