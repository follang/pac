#include "vm.h"

int vm_step(VmState *state, OpCode op, int operand)
{
    if (op == OP_PUSH) {
        state->stack[state->sp++] = operand;
        return operand;
    }
    if (op == OP_ADD) {
        state->stack[state->sp - 2u] += state->stack[state->sp - 1u];
        --state->sp;
        return state->stack[state->sp - 1u];
    }
    return 0;
}

int main(void)
{
    VmState vm = { { 0 }, 0u };
    vm_step(&vm, OP_PUSH, 4);
    vm_step(&vm, OP_PUSH, 5);
    return vm_step(&vm, OP_ADD, 0) == 9 ? 0 : 1;
}
