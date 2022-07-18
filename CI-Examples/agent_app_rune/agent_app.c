//#include "comm_lib.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <pthread.h>
#include <sched.h>
#include <assert.h>
#include <fcntl.h>
#include <errno.h>
#include <err.h>
#include <sys/stat.h>
#include "comm_lib.h"
#include "pf_util.h"

#define SOCK_TIMEOUT_SEC (1)

int sockfd;
int listener_thread_stop = 0;
void *payload_receiver_listener_1(void *ptr) {
	return NULL;
}

int write_output_file(const char* output_file, const char* buffer) {

    FILE *fptr = NULL;

    fptr = fopen(output_file,"w");
    if(fptr == NULL){
        printf("Error in opening output file %s failed...\n",output_file);
            return 1;
    }
    fwrite(buffer, strlen(buffer) ,1,fptr);
    fclose(fptr);

    printf("Successfuly created output protected file %s\n",output_file);

    return 0;
}


int main() {
	ErrorCode status;
	int connection_sockfd;
	CommInfo client_msg, msg;
	pid_t current_pid;
	pid_t payload_pid;
	pthread_t thread1;

	memset(&client_msg, 0, sizeof(client_msg));

	status = create_service(AGNET_DATA_PORT, &sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s create_service failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

	status = server_wait_for_client(sockfd, &connection_sockfd);
	if (status != OK) {
		fprintf(stderr, "%s:%d:%s server_wait_for_client failed %d\n", 
			__FILE__, __LINE__, __FUNCTION__, status);
		return -1;
	}

	printf("service enclave is listening to port 50051\n");
	while (1) {
		status = receive_command(connection_sockfd, &client_msg);
		if (status != OK) {
			fprintf(stderr, "%s:%d:%s receive_command failed %d\n", 
				__FILE__, __LINE__, __FUNCTION__, status);
			return -1;
		} else {
			printf("service enclave get command %d\n", client_msg.cmd);
		}

		switch (client_msg.cmd) {
			case APP_INIT:
				printf("service enclave got command APP_INIT %d\n", client_msg.cmd);	
				break;
			case GET_SECRETE:
				printf("service enclave got command GET_SECRETE %d\n", client_msg.cmd);
				
				write_key(NEW_KEY);
				char key_buf[16];
	        	read_key(KEY_PATH, key_buf, sizeof(key_buf));
	 	    	printf("YINGYING read back key = %s\n", key_buf);
				pf_encrypt_files ("helloworld", "helloworld_en", KEY_PATH);
#if 0 /* the followings are debugging code*/

				pf_decrypt_files("helloworld_en", "helloworld_new", true, KEY_PATH);
		        char *execute_path ="helloworld_new";

				char* const new_argv[] = {execute_path, NULL};
        		printf("buffer = %s\n", execute_path);
		        execv(execute_path, new_argv);
#endif	/*end of debugging code*/
				memset(&msg, 0, sizeof(msg));
				msg.cmd = GET_SECRETE_ACK;
				strcpy(msg.u.GetSecreteAckData.secrete, NEW_KEY);
				status = send_command(connection_sockfd, &msg);
				if (status != OK) {
					fprintf(stderr, "%s:%d:%s send_command GET_SECRETE_ACK failed %d\n", __FILE__, __LINE__, __FUNCTION__, status);
					return -1;
				} else {
					fprintf(stderr, "%s:%d:%s send_command GET_SECRETE_ACK successful\n", __FILE__, __LINE__, __FUNCTION__);
				}	
				break;
			case BYE:
				printf("service enclave command BYE %d\n", client_msg.cmd);
				status = close_connection(connection_sockfd);
				if (status != OK) {
					fprintf(stderr, "%s:%d:%s close_connection connection_sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, connection_sockfd, status);
					return -1;
				}
				status = close_connection(sockfd);
				if (status != OK) {
					fprintf(stderr, "%s:%d:%s close_connection sockfd %d failed %d\n", __FILE__, __LINE__, __FUNCTION__, sockfd, status);
					return -1;
				}
				return 0;
			default:
				printf("unknown command %d\n", client_msg.cmd);
		}
	}
}
