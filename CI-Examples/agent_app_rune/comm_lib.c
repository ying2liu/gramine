#include "comm_lib.h"
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
#include <errno.h>
#include <fcntl.h>
#include  <stddef.h>
#include <assert.h>
#include <err.h>

ssize_t posix_fd_rw(int fd, char* buf, size_t count, int do_write) {
    size_t transferred = 0;
    while (transferred < count) {
        ssize_t ret = do_write ? write(fd, buf + transferred, count - transferred) :
                                 read(fd, buf + transferred, count - transferred);

        if (ret < 0) {
            int ret_errno = errno;
            if (ret_errno == EINTR)
                continue;
            warn("%s", do_write ? "write" : "read");
            errno = ret_errno;
            return -1;
        }

        if (ret == 0) {
            /* end of file */
            break;
        }

        transferred += ret;
    }

    return transferred;
}

ssize_t posix_file_rw(const char* path, char* buf, size_t count, int do_write) {
    int fd = open(path, do_write ? O_WRONLY : O_RDONLY);
    if (fd < 0) {
        int ret_errno = errno;
        warn("open");
        errno = ret_errno;
        return -1;
    }

    ssize_t transferred = posix_fd_rw(fd, buf, count, do_write);
    if (transferred < 0) {
        int ret_errno = errno;
        int close_ret = close(fd);
        if (close_ret < 0)
            warn("close (during error handling)");
        errno = ret_errno;
        return -1;
    }

    int close_ret = close(fd);
    if (close_ret < 0) {
        int ret_errno = errno;
        warn("close");
        errno = ret_errno;
        return -1;
    }

    return transferred;
}

ssize_t posix_file_read(const char* path, char* buf, size_t count) {
    return posix_file_rw(path, buf, count, /*do_write=*/0);
}

ssize_t posix_file_write(const char* path, const char* buf, size_t count) {
    return posix_file_rw(path, (char*)buf, count, /*do_write=*/1);
}
int write_key(const char* key) {
    assert(strlen(key) == KEY_LEN);

    ssize_t bytes = posix_file_write(KEY_PATH, key, KEY_LEN);
    if (bytes < 0) {
        fprintf(stderr, "Error writing %s\n", KEY_PATH);
        return -1;
    }
    if ((size_t)bytes != KEY_LEN) {
        fprintf(stderr, "Failed: not enough bytes written to %s\n", KEY_PATH);
        return 1;
    }
    return 0;
}
int write_file(const char* path, char* buf, size_t count) {

    ssize_t bytes = posix_file_write(path, buf, count);
    if (bytes < 0) {
        fprintf(stderr, "Error writing %s\n", path);
        return -1;
    }
    if ((size_t)bytes != KEY_LEN) {
        fprintf(stderr, "Failed: not enough bytes written to %s\n", KEY_PATH);
        return 1;
    }
    return 0;
}


ssize_t read_key(const char* path, char* buf, size_t count) {
	ssize_t bytes = posix_file_read(KEY_PATH, buf, sizeof(buf));
    if (bytes < 0) {
        fprintf(stderr, "Error reading %s\n", KEY_PATH);
        return 1;
    }
#if 0
    if ((size_t)bytes != sizeof(buf)) {
        fprintf(stderr, "Failed: %s has wrong size\n", KEY_PATH);
        return 1;
    }
#endif
	return 0;
}

ErrorCode create_service(unsigned int port, int *sock_fd) {
	struct addrinfo *serverinfo = NULL, *p = NULL;
	int sockfd;

	int yes = 1;

	if (sock_fd == NULL) {
		return InvalidArgument;
	}
	sockfd = socket(AF_INET,     /* network versus AF_LOCAL */
	                  SOCK_STREAM, /* reliable, bidirectional, arbitrary payload size */
	                  0);          /* system picks underlying protocol (TCP) */
	if (sockfd < 0) {
		fprintf(stderr, "%s: server create socket failed %d/%s\n", __FUNCTION__, errno, strerror(errno));
		return CreateSocketErr;
	}

	if (setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof(int)) < 0) {
		fprintf(stderr, "%s: server setsockopt  SO_REUSEADDR failed %d/%s\n", 
			__FUNCTION__, errno, strerror(errno));
		return SetSockOptErr;
	}

  /* bind the server's local address in memory */
	struct sockaddr_in saddr;
  memset(&saddr, 0, sizeof(saddr));          /* clear the bytes */
  saddr.sin_family = AF_INET;                /* versus AF_LOCAL */
  saddr.sin_addr.s_addr = htonl(INADDR_ANY); /* host-to-network endian */
  saddr.sin_port = htons(port);        /* for listening */

	if (bind(sockfd, (struct sockaddr *) &saddr, sizeof(saddr)) < 0) {
		fprintf(stderr, "%s: server bind failed %d/%s\n", 
			__FUNCTION__, errno, strerror(errno));
		return CanNotBind;
	}

	*sock_fd = sockfd;
	return OK;
}

ErrorCode create_client(unsigned int port, int *sock_fd) {
	int sockfd;
	struct addrinfo *servinfo, *p;
	int rv;
	char s[INET6_ADDRSTRLEN];

	if (sock_fd == NULL) {
		return InvalidArgument;
	}
	sockfd = socket(AF_INET,      /* versus AF_LOCAL */
                      SOCK_STREAM,  /* reliable, bidirectional */
                      0);           /* system picks protocol (TCP) */
	if (sockfd < 0) {
		fprintf(stderr, "%s: client create socket failed %d/%s\n", 
			__FUNCTION__, errno, strerror(errno));
		return CreateSocketErr;
	}
     /* get the address of the host */
	struct hostent* hptr = gethostbyname(HOST_IP); /* localhost: 127.0.0.1 */
	if (!hptr) {
		fprintf(stderr, "%s: client gethostbyname failed\n", __FUNCTION__);
		return CreateSocketErr;
	}
	if (hptr->h_addrtype != AF_INET) {
		fprintf(stderr, "%s: client bad address family %d\n", 
			__FUNCTION__, hptr->h_addrtype);
		return BadAddressFamily;	
    }      /* versus AF_LOCAL */

    /* connect to the server: configure server's address 1st */
		struct sockaddr_in saddr;
		memset(&saddr, 0, sizeof(saddr));
		saddr.sin_family = AF_INET;
		saddr.sin_addr.s_addr =
		((struct in_addr*) hptr->h_addr_list[0])->s_addr;
    saddr.sin_port = htons(port); /* port number in big-endian */

		if (connect(sockfd, (struct sockaddr*) &saddr, sizeof(saddr)) < 0) {
			fprintf(stderr, "%s: client connect failed %d/%s\n", 
				__FUNCTION__, errno, strerror(errno));
			return ConnectErr;
		}
		printf("client: connected\n");
		*sock_fd = sockfd;

		return OK;
	}

	ErrorCode server_wait_for_client(int sock_fd, int *comm_socket) {
		struct sockaddr_storage client_addr;
		socklen_t sin_size;
		char s[INET6_ADDRSTRLEN];
		int new_fd;

		if (comm_socket == NULL) {
			return InvalidArgument;
		}

		if (listen(sock_fd, LISTEN_BACKLOG) == -1) {
			fprintf(stderr, "%s: listen failed %s\n", 
				__FUNCTION__, strerror(errno));
			return ListenErr;
		}	

		sin_size = sizeof(client_addr);
		new_fd = accept(sock_fd, (struct sockaddr *)&client_addr, &sin_size);
		if (new_fd == -1) {
			fprintf(stderr, "%s: accept failed %s\n", 
				__FUNCTION__, strerror(errno));
			return AcceptErr;
		}

		*comm_socket = new_fd;
		return OK;
	}

#if 0
	ErrorCode set_socket_timeout(int sock_fd, int timeout_sec) {
		struct timeval tv;
		tv.tv_sec = timeout_sec;
		tv.tv_usec = 0;
		int ret = setsockopt(sock_fd, 
			SOL_SOCKET, 
			SO_RCVTIMEO, 
			(const char*)&tv, 
			sizeof tv);
		if (ret != 0) {
			fprintf(stderr, "%s: setsockopt for timeout %ds %d\n", 
				__FUNCTION__, timeout_sec, ret);
			return SetOptErr;
		}
		return OK;
	}
#endif
	ErrorCode send_command(int comm_socket, CommInfo *cmd) {
		if (cmd == NULL) {
			return InvalidArgument;
		}
		if (send(comm_socket, cmd, sizeof(CommInfo), 0) == -1) {
			fprintf(stderr, "%s: send cmd %d failed %s\n", 
				__FUNCTION__, cmd->cmd, strerror(errno));
			return SendErr;
		}
		return OK;
	}

	ErrorCode receive_command(int comm_socket, CommInfo *cmd) {
		int numbytes;
		if (cmd == NULL) {
			return InvalidArgument;
		}
		// note: MSG_WAITALL is not supported in gramine
		if ((numbytes = recv(comm_socket, cmd, sizeof(CommInfo), 0 /*MSG_WAITALL*/)) == -1) {
		fprintf(stderr, "%s: receive cmd failed socket id= %d, %s\n", 
			__FUNCTION__, comm_socket, strerror(errno));
		return SendErr;
	}
	if (numbytes == 0) {
		return ReceiveTimeout;
	}
	return OK;
}

ErrorCode close_connection(int sock_fd) {
	close(sock_fd);
	return OK;
}

