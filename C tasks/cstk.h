//
// Sam Polyakov
// CS333
// cstk.h
//

#ifndef cstk.h
#define cstk.h

#include <stdio.h>

extern int MAX_STACK; 

typedef struct {
    int* stack;
    int top;
} Stack;

Stack *stk_create(int size); // creates stack of size
void stk_destroy(Stack* s); // deletes stack
void stk_push(Stack* s, int value); // adds to top of stack
int stk_pop(Stack* s); // removes from top of stack
void stk_display(Stack* s, int reverse); // prints out the list in the reverse order if the int value is 1, otherwise, prints out in the original order

#endif /* cstk.h */