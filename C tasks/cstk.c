//
// Sam Polyakov
// CS333
// cstk.c
//
// creates a stack using c
//

#include "cstk.h"
#include <stdlib.h>
#include <stdio.h>

int MAX_STACK = 100; // global variable


Stack *stk_create(int size) {
    Stack *s = (Stack*) malloc(sizeof(Stack));
    s->stack = (int*) malloc(size * sizeof(int));
    s->top = -1;
    return s;
}

void stk_destroy(Stack* s) {
    free(s->stack);
    free(s);
}

void stk_push(Stack* s, int value) {
    if (s->top >= MAX_STACK - 1) {
        printf("Stack overflow\n");
        return;
    }
    s->stack[++(s->top)] = value;
}

int stk_pop(Stack* s) {
    if (s->top < 0) {
        printf("Stack underflow\n");
        return -1;
    }
    return s->stack[(s->top)--];
}

void stk_display(Stack* s, int reverse) {
    if (reverse) {
        for (int i = s->top; i >= 0; i--) {
            printf("%d ", s->stack[i]);
        }
    } else {
        for (int i = 0; i <= s->top; i++) {
            printf("%d ", s->stack[i]);
        }
    }
    printf("\n");
}
