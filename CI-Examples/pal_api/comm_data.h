#ifndef COMM_DATA_H
#define COMM_DATA_H


#define MAX_FILE_PATH_LENGTH (256)
#define MAX_REPORT_SIZE (256)

#define COMM_DATA_PORT ("50051")

#define LISTEN_BACKLOG (10)

typedef enum {
	UNDEFINED = 0,
	APP_INIT,
	APP_EXECUTE, // using ExecuteData
	APP_EXECUTE_ACK, // from server back to client, using ExcecuteAckData
	APP_KILL,
	APP_DESTROY,
	GET_REPORT,
	GET_REPORT_ACK, // from server to client, using GetReportAckData
	BYE,
	MAX_MSG_ID
} CommCommand;

typedef struct {
	CommCommand cmd;
	union {
		struct {
			char name[MAX_FILE_PATH_LENGTH];
		} ExecuteData;
		struct {
			int pid;
		} ProcessData;
		struct {
			char report[MAX_REPORT_SIZE];
		} GetReportAckData;
	} u;
} CommInfo;

typedef enum {
	OK = 0,
	InvalidArgument,
	GetAddrInfoErr,
	SetSockOptErr,
	CanNotBind,
	ListenErr,
	AcceptErr,
	ConnectErr,
	SendErr,
	ReceiveErr,
} ErrorCode;

extern ErrorCode createService(int *sock_fd);
extern ErrorCode createClient(int *sock_fd);
extern ErrorCode serverWaitForClient(int sock_fd, int *comm_socket);

extern ErrorCode sendCommand(int comm_socket, CommInfo *cmd);
extern ErrorCode receiveCommand(int comm_socket, CommInfo *cmd);

extern ErrorCode closeConnection(int sock_fd);

#endif
