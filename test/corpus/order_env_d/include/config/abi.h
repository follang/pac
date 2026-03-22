#ifndef PARC_CORPUS_ORDER_D_ABI_H
#define PARC_CORPUS_ORDER_D_ABI_H

#include <limits.h>
#include <stdarg.h>

#if !defined(ORDER_PTR_WIDE)
#  error "config/core.h must be included before config/abi.h"
#endif

#if ULONG_MAX > 0xffffffffUL
#  define ORDER_ENUM_BASE(n) ((n) + ORDER_PTR_WIDE)
#else
#  define ORDER_ENUM_BASE(n) (n)
#endif

typedef struct order_handle order_handle;

#endif
