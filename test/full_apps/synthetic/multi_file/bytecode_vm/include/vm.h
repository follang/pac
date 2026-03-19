#ifndef VM_H
#define VM_H

#include "vm_types.h"

int vm_step(VmState *state, OpCode op, int operand);

#endif
