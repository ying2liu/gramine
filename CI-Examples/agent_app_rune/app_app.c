#include "comm_lib.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "pf_util.h"

ErrorCode cleanup(int agent_sockfd, int rune_comm_sock_fd, int rune_sockfd) {
	// wait till rune_app to finish
	sleep(3);

	ErrorCode status = close_connection(agent_sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s close_connection agent_sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, agent_sockfd, status);
		return status;
	}

	status = close_connection(rune_comm_sock_fd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s close_connection rune_comm_sock_fd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, rune_comm_sock_fd, status);
		return status;
	}

	status = close_connection(rune_sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s close_connection rune_sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, rune_sockfd, status);
		return status;
	}
	return OK;
}

int main() {
	int rune_sockfd, rune_comm_sock_fd, agent_sockfd;
	CommInfo msg;
	ErrorCode status;
	char *execute_path;

	// create server for rune_app
	status = create_service(APP_DATA_PORT, &rune_sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s create_service for app failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s create_service successful\n",
			__FILE__, __LINE__, __FUNCTION__);
	}

	status = server_wait_for_client(rune_sockfd, &rune_comm_sock_fd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s server_wait_for_client for app failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s server_wait_for_client successful\n",
			__FILE__, __LINE__, __FUNCTION__);
	}

	// create client to agent connection
	status = create_client(AGNET_DATA_PORT, &agent_sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s create_client for AGNET_DATA_PORT failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s for agent successful\n",
			__FILE__, __LINE__, __FUNCTION__);
	}

	// to get secrete from agent
	memset(&msg, 0, sizeof(msg));
	msg.cmd = GET_SECRETE;
	status = send_command(agent_sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s send_command GET_SECRETE failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s send_command GET_SECRETE successful\n",
			__FILE__, __LINE__, __FUNCTION__);
	}

	status = receive_command(agent_sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s receive_command failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("get command %d\n", msg.cmd);
	}
	if (msg.cmd != GET_SECRETE_ACK) {
		fprintf(stderr, "%s:%d:%s receive_command failed to get GET_SECRETE_ACK %d\n",
			__FILE__, __LINE__, __FUNCTION__, msg.cmd);
		return -1;
	} else {
        printf("after APP_EXECUTE get GET_SECRETE_ACK with secret %s\n",
        msg.u.GetSecreteAckData.secrete);
		write_key(KEY_PATH, &msg.u.GetSecreteAckData.secrete);
	}

	// send bye to agent
	msg.cmd = BYE;
	status = send_command(agent_sockfd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s send_command BYE failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		fprintf(stderr, "%s:%d:%s send_command BYE successful\n",
			__FILE__, __LINE__, __FUNCTION__);
	}


	// now wait for APP_EXECUTE/APP_KILL from rune_app
	status = receive_command(rune_comm_sock_fd, &msg);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s receive_command failed %d\n",
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	} else {
		printf("get command %d\n", msg.cmd);
	}
	switch (msg.cmd) {
		case APP_EXECUTE: {
			size_t execute_path_len = strlen(msg.u.ExecuteData.name);
            execute_path = malloc(execute_path_len);
			memset (execute_path, 0, execute_path_len);
			memcpy(execute_path, msg.u.ExecuteData.name, execute_path_len);
			printf("msg.u.ExecuteData.name=%s\n", msg.u.ExecuteData.name);
				// get APP_EXECUTE to send APP_EXECUTE_ACK
				// back, clean up, then execute
			memset(&msg, 0, sizeof(msg));
			msg.cmd = APP_EXECUTE_ACK;
			msg.u.ProcessData.pid = 123;
			status = send_command(rune_comm_sock_fd, &msg);
			if (status != OK) {
				fprintf(stderr, "%s:%d:%s send_command APP_EXECUTE_ACK failed %d\n",
					__FILE__, __LINE__, __FUNCTION__, status);
				return -1;
			}
			else {
				fprintf(stderr, "%s:%d:%s send_command APP_EXECUTE_ACK successful\n",
					__FILE__, __LINE__, __FUNCTION__);
			}

			status = cleanup(agent_sockfd, rune_comm_sock_fd, rune_sockfd);
			if (status != OK) {
				return -1;
			}

			if (pf_init() != 0) {
				fprintf(stderr, "%s:%d:%s Failed to initialize protected files\n", __FILE__,
								__LINE__, __FUNCTION__);
				return -1;
			}

            pf_decrypt_files("enc/test_encrypt.txt", "test_decrypt.txt", false, KEY_PATH);
            char* const new_argv[] = {execute_path, NULL};
            printf("buffer = %s\n", execute_path);
            execv(execute_path, new_argv);

			fprintf(stderr, "%s:%d:%s to execute\n", __FILE__, __LINE__, __FUNCTION__);
		}
		break;
		case APP_KILL: {
				// get APP_EXECUTE to send APP_EXECUTE_ACK
				// back, clean up, then execute
			memset(&msg, 0, sizeof(msg));
			msg.cmd = APP_KILL_ACK;
				//msg.u.ProcessData.pid = kill_pid;
			status = send_command(rune_comm_sock_fd, &msg);
			if (status != OK) {
				fprintf(stderr, "%s:%d:%s send_command APP_KILL_ACK failed %d\n",
					__FILE__, __LINE__, __FUNCTION__, status);
				return -1;
			} else {
				fprintf(stderr, "%s:%d:%s send_command APP_KILL_ACK successful\n",
					__FILE__, __LINE__, __FUNCTION__);
			}
			status = cleanup(agent_sockfd, rune_comm_sock_fd, rune_sockfd);
			if (status != OK) {
				return -1;
			}

			/*
			* TODO: implement kill
			*/
			fprintf(stderr, "%s:%d:%s to kill\n", __FILE__, __LINE__, __FUNCTION__);
		}
		break;
		default: {
			fprintf(stderr, "%s:%d:%s receive_command from rune_app unrecognized %d\n",
				__FILE__, __LINE__, __FUNCTION__, msg.cmd);
			return -1;
		}
	}

	return 0;
}
