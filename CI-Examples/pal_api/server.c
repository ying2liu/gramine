#include "comm_data.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>

int main() {
	ErrorCode status;
	int sockfd, connection_sockfd;
	CommInfo clientMsg, msg;
    pid_t current_pid;
    pid_t payload_pid;

   	memset(&clientMsg, 0, sizeof(clientMsg));

	status = createService(&sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s createService failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

	status = serverWaitForClient(sockfd, &connection_sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s createService failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

    printf("server is listening to port 50051\n");
	while (1) {
		status = receiveCommand(connection_sockfd, &clientMsg);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
			return -1;
		} else {
			printf("get command %d\n", clientMsg.cmd);
		}

		switch (clientMsg.cmd) {
			case APP_INIT:
				printf("get command APP_INIT %d\n", clientMsg.cmd);	
				current_pid = fork();
                char* const new_argv[] = {"payload_receiver", (char*)"test", "./helloworld"};
                if (current_pid == 0) {
                    execv("payload_receiver", new_argv);
                }
                break;
			case APP_EXECUTE:
				printf("get command APP_EXECUTE %d with path %s\n", clientMsg.cmd, clientMsg.u.ExecuteData.name);	
                char* const new_argv1[] = {"helloworld", NULL};
                if (current_pid == 0) {
                    printf("YINGYING will run helloworld\n");
                    execv("helloworld", new_argv1);
                }
				memset(&msg, 0, sizeof(msg));
				msg.cmd = APP_EXECUTE_ACK;
				msg.u.ProcessData.pid = current_pid;
				status = sendCommand(connection_sockfd, &msg);
				if (status != OK) {
					fprintf(stderr, "%s:%d:%s sendCommand APP_EXECUTE_ACK failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
					return -1;
				}
				break;
			case APP_KILL:
                sleep(10);
				printf("get command KILL %d, %d\n", clientMsg.cmd, clientMsg.u.ProcessData.pid);
                kill(clientMsg.u.ProcessData.pid, SIGKILL);
				
				return 0;
		    case BYE:
				printf("get command BYE %d\n", clientMsg.cmd);
				status = closeConnection(connection_sockfd);
				if (status != OK) {
					fprintf(stderr, "%s:%d:%s closeConnection connection_sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, connection_sockfd, status);
					return -1;
				}
				status = closeConnection(sockfd);
				if (status != OK) {
					fprintf(stderr, "%s:%d:%s closeConnection sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, sockfd, status);
					return -1;
				}
				return 0;
default:
				printf("unknown command %d\n", clientMsg.cmd);
		}
	}
}
