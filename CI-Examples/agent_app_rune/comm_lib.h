#ifndef COMM_LIB_H
#define COMM_LIB_H
#include <unistd.h>
#pragma once

#define MAX_FILE_PATH_LENGTH (256)
#define MAX_REPORT_SIZE (256)
#define MAX_SECRETE_SIZE (256)

#define HOST_IP	"127.0.0.1"  //local host


#define COMM_TEST_DATA_PORT (50051)
#define AGNET_DATA_PORT (50051) // between agent/app, agent is server
#define APP_DATA_PORT (50053) // between app and rune, app is server

#define LISTEN_BACKLOG (10)

typedef enum {
	UNDEFINED = 0,
	APP_INIT,
	APP_EXECUTE, // using ExecuteData
	APP_EXECUTE_ACK, // from server back to client, using ExcecuteAckData
	APP_KILL,
	APP_KILL_ACK,
	APP_DESTROY,
	GET_REPORT,
	GET_REPORT_ACK, // from server to client, using GetReportAckData
	APP_TERMINATE,
	STATUS,
	APP_STATUS_ACK,
	GET_SECRETE,
	GET_SECRETE_ACK,
	BYE,
	MAX_MSG_ID
} CommCommand;

typedef struct {
	CommCommand cmd;
	union {
		struct {
			char name[MAX_FILE_PATH_LENGTH];
			char parameter[128];
			int avail;
		} ExecuteData;
		struct {
			int pid;
		} ProcessData;
		struct {
			char report[MAX_REPORT_SIZE];
		} GetReportAckData;
		struct {
			char secrete[MAX_SECRETE_SIZE];
		} GetSecreteAckData;		
		struct {
			int pid;
			int sig;
		} KillData;
		struct {
			int pid;
			int sig;
			int status; // return value of posix kill
		} KillAckData;		
	} u;
} CommInfo;

typedef enum {
	OK = 0,
	InvalidArgument,
	CreateSocketErr,
	GetAddrInfoErr,
	SetSockOptErr,
	CanNotBind,
	ListenErr,
	AcceptErr,
	GetHostNameErr,
	BadAddressFamily,
	ConnectErr,
	SetOptErr,
	SendErr,
	ReceiveErr,
	ReceiveTimeout,
} ErrorCode;

extern ErrorCode create_service(unsigned int port, int *sock_fd);
extern ErrorCode create_client(unsigned int port, int *sock_fd);
extern ErrorCode server_wait_for_client(int sock_fd, int *comm_socket);
extern ErrorCode set_socket_timeout(int sock_fd, int timeout_sec);

extern ErrorCode send_command(int comm_socket, CommInfo *cmd);
extern ErrorCode receive_command(int comm_socket, CommInfo *cmd);

extern ErrorCode close_connection(int sock_fd);
#define KEY_LEN 16
#define NEW_KEY      "0011223344556677"
#define KEY_PATH "/dev/attestation/keys/default"
ssize_t posix_fd_rw(int fd, char* buf, size_t count, int do_write); 
ssize_t posix_file_rw(const char* path, char* buf, size_t count, int do_write); 
int write_key(const char* key);
ssize_t read_key(const char* path, char* buf, size_t count);
ssize_t posix_file_write(const char* path, const char* buf, size_t count);
int write_file(const char* path, char* buf, size_t count);
#endif //COMM_LIB_H
