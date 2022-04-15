#include "comm_data.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
	int sockfd;
	CommInfo msg, serviceMsg;
	ErrorCode status;
	char *execute_path ="helloworld";

	status = createClient(&sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s createClient failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}
	memset(&msg, 0, sizeof(msg));
	msg.cmd = APP_INIT;
	status = sendCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s sendCommand APP_INIT failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

	msg.cmd = APP_EXECUTE;
	memcpy(&msg.u.ExecuteData.name[0], execute_path, strlen(execute_path));
	status = sendCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s sendCommand APP_EXECUTE failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}
	status = receiveCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s receiveCommand failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("get command %d\n", msg.cmd);
	}
	if (msg.cmd != APP_EXECUTE_ACK) {
		fprintf(stderr, "%s:%d:%s receiveCommand failed to get APP_EXECUTE_ACK %d\n", __FILE__, __LINE__, __FUNCTION__, msg.cmd);
		return -1;
	} else {
		printf("after APP_EXECUTE get APP_EXECUTE_ACK with pid %d\n", msg.u.ProcessData.pid);
	}
    int existing_pid = msg.u.ProcessData.pid;

	msg.cmd = APP_DESTROY;
	status = sendCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s sendCommand APP_EXECUTE failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}


	msg.cmd = BYE;
	status = sendCommand(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s sendCommand APP_EXECUTE failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

	sleep(1);

	status = closeConnection(sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s closeConnection sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, sockfd, status);
		return -1;
	}

	return 0;	
}
