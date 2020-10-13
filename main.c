#include <stdio.h>

void foo1(int a) {
    printf("%d\n", a);
}

int foo2(int a, int b) {
    return a + 1;
}

int foo3(int x, int y) {
    return x / 2;
}

int main() {
    // // constant folding
    // int a = 10000;
    // for(int i=0; i<100; i++){
    //     a += i;
    // }
    // printf("%d\n", a);

    // Canonicalize Induction Variables
    // for (int i = 7; i * i < 10000; ++i) {
    //     printf("%d\n", i);
    // }

    // int z = x + y;
    // printf("%d\n", z);
}
