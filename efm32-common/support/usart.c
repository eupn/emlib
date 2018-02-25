#include "em_usart.h"

USART_TypeDef* GET_USART0() {
    return USART0;
}

USART_TypeDef* GET_USART1() {
    return USART1;
}

void STATIC_INLINE_USART_IntClear(USART_TypeDef *usart, uint32_t flags) {
    USART_IntClear(usart, flags);
}

void STATIC_INLINE_USART_IntEnable(USART_TypeDef *usart, uint32_t flags) {
    USART_IntEnable(usart, flags);
}
