#define _XOPEN_SOURCE 700
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main(int argc, char** argv) {
    pid_t child_pid;
    int newfd = dup(1);
    if (newfd < 0) {
        perror("dup failed");
        return 1;
    }

    char fd_argv[12];
    snprintf(fd_argv, 12, "%d", newfd);
    char* const new_argv[] = {(char*)"./helloworld", fd_argv, NULL};
    
    child_pid = fork();
    int flag = 1;

    while (1) {
        if ((child_pid == 0) && (flag == 1)){
            execv(new_argv[0], new_argv);
        } 
   }
   
    puts("test completed successfully");
    return 0;
}
