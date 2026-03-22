#ifndef PARC_CORPUS_ORDER_D_API_H
#define PARC_CORPUS_ORDER_D_API_H

#if !defined(ORDER_ENUM_BASE)
#  error "config/abi.h must be included before api.h"
#endif

typedef enum order_mode {
    ORDER_MODE_TINY = ORDER_ENUM_BASE(1),
    ORDER_MODE_WIDE = ORDER_ENUM_BASE(2)
} order_mode;

typedef struct order_packet {
    order_word_t flags;
    order_size_t size;
    order_mode mode;
} order_packet;

ORDER_API order_handle *ORDER_CALL order_open(const order_packet *packet);
ORDER_API int ORDER_CALL order_log(order_handle *handle, const char *fmt, va_list ap);

#endif
