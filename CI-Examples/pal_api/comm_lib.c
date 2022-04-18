#include "comm_data.h"
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <unistd.h> //for close
#include <netdb.h> // for gai_strerror
#include <string.h> // for strerror
#include <arpa/inet.h> // for inet_ntop
#include <netinet/in.h> // for get_in_addr

ErrorCode createService(int *sock_fd) {
	struct addrinfo hints, *serverinfo = NULL, *p = NULL;
	int sockfd;

	int yes=1;

	int ret = 0;

	if (sock_fd == NULL) {
		return InvalidArgument;
	}

	memset(&hints, 0, sizeof(hints));
	hints.ai_family = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;
	hints.ai_flags = AI_PASSIVE; // use my ip

	if ((ret = getaddrinfo(NULL /*INADDR_ANY*/, COMM_DATA_PORT, &hints, &serverinfo)) != 0) {
		fprintf(stderr, "%s: server getaddrinfo failed %d/%s\n", __FUNCTION__, ret, gai_strerror(ret));
		return GetAddrInfoErr;
	}

	// loop through all the results and bind to the first we can
	for(p = serverinfo; p != NULL; p = p->ai_next) {
		if ((sockfd = socket(p->ai_family, p->ai_socktype,
				p->ai_protocol)) == -1) {
			continue;
		}

		if (setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &yes,
				sizeof(int)) == -1) {
			fprintf(stderr, "%s: setsockopt %s\n", __FUNCTION__, strerror(errno));
			return SetSockOptErr;
		}

		if (bind(sockfd, p->ai_addr, p->ai_addrlen) == -1) {
			close(sockfd);
			continue;
		}

		break;
	}
	freeaddrinfo(serverinfo);

	if (p == NULL) {
		fprintf(stderr, "%s can not bind\n", __FUNCTION__);
		return CanNotBind;
	}
	*sock_fd = sockfd;
	return OK;
}

ErrorCode createClient(int *sock_fd) {
	int sockfd;
	struct addrinfo hints, *servinfo, *p;
	int rv;
	char s[INET6_ADDRSTRLEN];

	if (sock_fd == NULL) {
		return InvalidArgument;
	}

	memset(&hints, 0, sizeof(hints));
	hints.ai_family = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;

	if ((rv = getaddrinfo(NULL/*INADDR_LOOPBACK*/, COMM_DATA_PORT, &hints, &servinfo)) != 0) {
		fprintf(stderr, "%s: clinet getaddrinfo: %s\n", __FUNCTION__, gai_strerror(rv));
		return GetAddrInfoErr;
	}

	// loop through all the results and connect to the first we can
	for(p = servinfo; p != NULL; p = p->ai_next) {
		if ((sockfd = socket(p->ai_family, p->ai_socktype,
				p->ai_protocol)) == -1) {
			continue;
		}

		if (connect(sockfd, p->ai_addr, p->ai_addrlen) == -1) {
			close(sockfd);
			continue;
		}

		break;
	}

	freeaddrinfo(servinfo); // all done with this structure

	if (p == NULL) {
		fprintf(stderr, "%s client: failed to connect\n", __FUNCTION__);
		return ConnectErr;
	}

	printf("client: connected\n");
	*sock_fd = sockfd;

	return OK;
}

ErrorCode serverWaitForClient(int sock_fd, int *comm_socket) {
	struct sockaddr_storage client_addr;
	socklen_t sin_size;
	char s[INET6_ADDRSTRLEN];
	int new_fd;

	if (comm_socket == NULL) {
		return InvalidArgument;
	}

	if (listen(sock_fd, LISTEN_BACKLOG) == -1) {
		fprintf(stderr, "%s: listen failed %s\n", __FUNCTION__, strerror(errno));
		return ListenErr;
	}	

	sin_size = sizeof(client_addr);
	new_fd = accept(sock_fd, (struct sockaddr *)&client_addr, &sin_size);
	if (new_fd == -1) {
		fprintf(stderr, "%s: accept failed %s\n", __FUNCTION__, strerror(errno));
		return AcceptErr;
	}

	*comm_socket = new_fd;
	return OK;
}

ErrorCode sendCommand(int comm_socket, CommInfo *cmd) {
	if (cmd == NULL) {
		return InvalidArgument;
	}
	if (send(comm_socket, cmd, sizeof(CommInfo), 0) == -1) {
		fprintf(stderr, "%s: send cmd %d failed %s\n", __FUNCTION__, cmd->cmd, strerror(errno));
		return SendErr;
	}
	return OK;
}

ErrorCode receiveCommand(int comm_socket, CommInfo *cmd) {
	int numbytes;
	if (cmd == NULL) {
		return InvalidArgument;
	}
	if ((numbytes = recv(comm_socket, cmd, sizeof(CommInfo), MSG_WAITALL)) == -1) {
		fprintf(stderr, "%s: receive cmd failed socket id= %d, %s\n", __FUNCTION__, comm_socket, strerror(errno));
		return SendErr;
	}
	
	return OK;
}

ErrorCode closeConnection(int sock_fd) {
	close(sock_fd);
	return OK;
}
