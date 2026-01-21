#include <stdio.h>
#include <time.h>

int main() {
    time_t t;
    time(&t);
    printf("Hello Farveen, right now the time is %s", ctime(&t));
    return 0;
}
