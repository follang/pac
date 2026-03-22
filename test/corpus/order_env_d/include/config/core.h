#ifndef PARC_CORPUS_ORDER_D_CORE_H
#define PARC_CORPUS_ORDER_D_CORE_H

#include <stddef.h>
#include <stdint.h>

#define ORDER_API extern
#define ORDER_CALL

#if __SIZEOF_POINTER__ >= 8
#  define ORDER_PTR_WIDE 1
typedef uint64_t order_word_t;
#else
#  define ORDER_PTR_WIDE 0
typedef uint32_t order_word_t;
#endif

typedef size_t order_size_t;

#endif
