#include "comm_data.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <pthread.h>
#include <sched.h>

int sockfd, payload_receiver_1 = -1;
int payload_receiver1_status = -1;
void *payload_receiver_listener_1(void *ptr) {
	ErrorCode status;
	CommInfo clientMsg, msg;

       printf("payload_receiver_server_listener_1 is listening to payload_receiver_1\n");
        while (1) {
                status = receiveCommand(payload_receiver_1, &clientMsg);
                if (status != OK) {
                        fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
                }
                switch (clientMsg.cmd) {
                        case STATUS:
				payload_receiver1_status = clientMsg.u.ExecuteData.avail;
				printf("payload_receiver_server_listener_1 got command STATUS clientMsg.u.ExecuteData.avail= %d\n",clientMsg.u.ExecuteData.avail);
				break;
			default:
				break;
		}
	}
}

int main() {
	ErrorCode status;
	int connection_sockfd;
	CommInfo clientMsg, msg;
	pid_t current_pid;
        pid_t payload_pid;
	pthread_t thread1;

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

        printf("service enclave is listening to port 50051\n");
	while (1) {
		status = receiveCommand(connection_sockfd, &clientMsg);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
			return -1;
		} else {
			printf("service enclave get command %d\n", clientMsg.cmd);
		}

		switch (clientMsg.cmd) {
			case APP_INIT:
				printf("service enclave got command APP_INIT %d\n", clientMsg.cmd);	
				current_pid = fork();
	//	                char* const new_argv[] = {"payload_receiver", (char *)"test", "./helloworld"};
        	                char* const new_argv[] = {"payload_receiver", NULL};
        	 		if (current_pid == 0) {
                   			 execv("payload_receiver", new_argv);
                		} else {
					status = serverWaitForClient(sockfd, &payload_receiver_1);
				        if (status != OK) {
               					fprintf(stderr, "%s:%d:%s createService failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
					}
 
					int x = pthread_create(&thread1, NULL,  payload_receiver_listener_1, "payload_receiver 1");
		            		if (x != 0) {
                				printf("pthread_create: %d\n", x);
           				 }
				}
                		break;
			case APP_EXECUTE:
				printf("service enclave got command APP_EXECUTE %d with path %s payload_recever1_status=%d\n", clientMsg.cmd, clientMsg.u.ExecuteData.name, payload_receiver1_status);	
        			while (payload_receiver1_status == 0) {
					sched_yield();
				}
				status = sendCommand(payload_receiver_1, &clientMsg);
				if (status != OK) {
		                	fprintf(stderr, "%s:%d:%s YINGYING sendCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);

				}
        			while (payload_receiver1_status == 1) {
					sched_yield();
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
			case APP_DESTROY:
		                sleep(10);
				printf("service enclave command KILL %d, %d\n", clientMsg.cmd, clientMsg.u.ProcessData.pid);
                		kill(clientMsg.u.ProcessData.pid, SIGKILL);
				
				return 0;
		    	case BYE:
				printf("service enclave command BYE %d\n", clientMsg.cmd);
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
