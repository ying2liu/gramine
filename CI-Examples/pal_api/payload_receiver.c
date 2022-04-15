#include "comm_data.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
	int sockfd;
	CommInfo msg, newmsg, serviceMsg;
	ErrorCode status;

	status = createClient(&sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s createClient failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("payload_receiver was connected to server\n");
	}
	memset(&msg, 0, sizeof(msg));
	msg.cmd = STATUS;
	msg.u.ExecuteData.avail = 1;
	status = sendCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s sendCommand STATUS failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("payload_receiver sent out status %d\n", msg.u.ExecuteData.avail);
	}

	while (1) {
		status = receiveCommand(sockfd, &msg);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
			return -1;
		} else {
			printf("payload_receiver received binary %s\n", msg.u.ExecuteData.name);
		}

		newmsg.cmd = STATUS;
        	newmsg.u.ExecuteData.avail = 0;;
		status = sendCommand(sockfd, &newmsg);
        	if (status != OK) {
                	fprintf(stderr, "%s:%d:%s sendCommand STATUS failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
                	return -1;
        	} else {
			printf("payload_receiver sent out status %d\n", newmsg.u.ExecuteData.avail);
		}
	        char* const new_argv[] = {msg.u.ExecuteData.name, (char*)"test", msg.u.ExecuteData.name};
                execv(msg.u.ExecuteData.name, new_argv);
		status = closeConnection(sockfd);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s closeConnection sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, sockfd, status);
			return -1;
		}
	}
	
	return 0;	
}
