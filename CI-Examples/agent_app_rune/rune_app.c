#include "comm_lib.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
	int sockfd;
	CommInfo msg;
	ErrorCode status;
	char *execute_path ="helloworld";
	while (1) {
		status = create_client(APP_DATA_PORT, &sockfd);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s create_client failed %d\n", 
				__FILE__, __LINE__, __FUNCTION__, status);
			continue;
		} else {
			fprintf(stderr, "%s:%d:%s create_client for PP_DATA_PORT successful \n", 
				__FILE__, __LINE__, __FUNCTION__);	
			break;
		}
		sleep(3);
	}
	memset(&msg, 0, sizeof(msg));
	msg.cmd = APP_EXECUTE;
	memcpy(&msg.u.ExecuteData.name[0], execute_path, strlen(execute_path));
	status = send_command(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s send_command APP_EXECUTE failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s send_command APP_EXECUTE successful\n", 
			__FILE__, __LINE__, __FUNCTION__);
	}
	
	status = receive_command(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s receive_command failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("get command %d\n", msg.cmd);
	}
	
	if (msg.cmd != APP_EXECUTE_ACK) {
		fprintf(stderr, "%s:%d:%s receive_command failed to get APP_EXECUTE_ACK %d\n", __FILE__, __LINE__, __FUNCTION__, msg.cmd);
		return -1;
	} else {
		printf("after APP_EXECUTE get APP_EXECUTE_ACK with pid %d\n", msg.u.ProcessData.pid);
	}
	int existing_pid = msg.u.ProcessData.pid;
#if 0
	msg.cmd = APP_KILL;
	msg.u.KillData.pid = existing_pid;
	msg.u.KillData.sig = 9;
	status = send_command(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s send_command APP_KILL failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s send_command APP_KILL successful\n", 
			__FILE__, __LINE__, __FUNCTION__);
	}
	status = receive_command(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s receive_command failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("get command %d\n", msg.cmd);
	}
	if (msg.cmd != APP_KILL_ACK) {
		fprintf(stderr, "%s:%d:%s receive_command failed to get APP_KILL_ACK %d\n", 
			__FILE__, __LINE__, __FUNCTION__, msg.cmd);
		return -1;
	} else {
		printf("after APP_EXECUTE get APP_KILL_ACK with pid %d status %d\n", 
			msg.u.KillAckData.pid, msg.u.KillAckData.status);
	}

	msg.cmd = BYE;
	status = send_command(sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s send_command APP_EXECUTE failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

	sleep(1);
#endif
	status = close_connection(sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s close_connection sockfd %d failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, sockfd, status);
		return -1;
	}

	return 0;	
}
