#include "comm_data.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
	int sockfd;
	CommInfo msg, msg1;
	ErrorCode status;

	status = createClient(&sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s createClient failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("payload_receiver was connected to its listener\n");
	}
	memset(&msg, 0, sizeof(msg));
	msg.cmd = STATUS;
	msg.u.ExecuteData.avail = 1;
	status = sendCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s sendCommand STATUS failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} 

	while (1) {
		status = receiveCommand(sockfd, &msg);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
			return -1;
		} else {
			printf("payload receiver got cmd msg.cmd = %d\n", msg.cmd);
		}
		switch (msg.cmd) {
			case APP_EXECUTE:;
				char* const new_argv[] = {msg.u.ExecuteData.name, (char*)"test", msg.u.ExecuteData.name};
				msg1.cmd = STATUS;
		        	msg1.u.ExecuteData.avail = 0;;
				status = sendCommand(sockfd, &msg1);
		        	if (status != OK) {
                			fprintf(stderr, "%s:%d:%s sendCommand STATUS failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
                			return -1;
        			} else {
					printf("receiver sent status = 0\n");
				}

				while (1) {
					status = receiveCommand(sockfd, &msg1);
        				if (status != OK) {
				                fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
        	       				 return -1;
        				} else {
						if (msg1.cmd != APP_EXECUTE_ACK) {
							printf("payload_receiver got status_ack\n");
							status = closeConnection(sockfd);
							execv(msg.u.ExecuteData.name, new_argv);
		         				break;
						}
	        			}	
				}
				break;
			case APP_TERMINATE:
				msg.cmd = BYE;
				status = sendCommand(sockfd, &msg);
			        if (status != OK) {
               				 fprintf(stderr, "%s:%d:%s sendCommand APP_TERMINATE failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
			                return -1;
			        }
				break;
			default:
				break;
		}
	}
	
	return 0;	
}			

