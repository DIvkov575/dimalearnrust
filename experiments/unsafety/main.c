#include <stdio.h>

int main() {
    int a = 5;
    int *b = &a;
    int *c = b;

    *b += 1;


    printf("%d", *b);

}