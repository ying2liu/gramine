#define _XOPEN_SOURCE 700
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main(int argc, char** argv) {
    int flag = 1;
    char* const new_argv[] = {(char*)"./helloworld", "helloworld", NULL};

    printf("hello from payload_receiver\n");
    while (1) {
        if (flag == 1) {
            execv(new_argv[0], new_argv);
        } 
   }
   
    printf("test completed successfully");
    return 0;
}
