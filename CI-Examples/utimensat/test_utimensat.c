#include <stdio.h>

#include <utime.h>
#include <sys/time.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#ifndef _GNU_SOURCE
# define _GNU_SOURCE
#endif

static int timeval_to_timespec(const struct timeval *tv, struct timespec *ts) {
    if (tv != NULL && ts != NULL) {
        ts->tv_sec = tv->tv_sec;
        ts->tv_nsec = tv->tv_usec * 1000;
        return 0;
    } else {
        return -1;
    }
}

int main(void) {
    struct timespec times[2];
    struct timeval tv[2];
    struct utimbuf utb;
    int fd = open("/yingtmp/test", O_RDWR, 0777);

    if (fd < 0) {
        printf("open file failed, fd=%d\n", fd);
    }
    if (utimensat(/*AT_FDCWD*/fd, "/yingtmp/test",NULL, 0) < 0) {
        printf("utimensat times = NULL, failed\n");
	} else {
        printf("utimensat times = NULL succeeded\n");
    }
   if (utimensat(AT_FDCWD, "./yingtmp/test",NULL, 0) < 0) {
        printf("utimensat relative path, times = NULL, failed\n");
	} else {
        printf("utimensat relative times = NULL succeeded\n");
    }


    for (int i = 0; i < 2; i ++) {
        gettimeofday(&tv[i], NULL);
        timeval_to_timespec(&tv[i], &times[i]);
    }
    printf("times[0].tv_sec=%ld, times[0].tv_nsec=%ld\n", times[0].tv_sec, times[0].tv_nsec);
    int ret = utimensat(/*AT_FDCWD*/fd, "/yingtmp/test", times, 0);
    if (ret < 0) {
        printf("utimensat failed, ret = %d\n", ret);
	} else { 
        printf("utimensat succeeded\n");
    }
    ret = utimensat(AT_FDCWD, "./yingtmp/test", times, 0);
    if (ret < 0) {
        printf("utimensat relative failed, ret = %d\n", ret);
	} else { 
        printf("utimensat relative succeeded\n");
    }

    ret = futimesat(/*AT_FDCWD*/fd, "/yingtmp/test", tv);
    if (ret < 0) {
        printf("futimesat failed, ret = %d\n", ret);
	} else { 
        printf("futimesat succeeded\n");
    }
    ret = futimesat(AT_FDCWD, "./yingtmp/test", tv);
    if (ret < 0) {
        printf("futimesat relative failed, ret = %d\n", ret);
	} else { 
        printf("futimesat relative succeeded\n");
    }

#if 0
    ret = futimens(/*AT_FDCWD*/fd, times);
    if (ret < 0) {
        printf("futimen failed, ret = %d\n", ret);
	} else { 
        printf("futimen succeeded\n");
   }
#endif    
#if 1
    ret = utimes("/yingtmp/test", tv);
    if (ret < 0) {
        printf("utimes failed, ret = %d\n", ret);
	} else { 
        printf("utimes succeeded\n");
    }
    utb.actime = time((time_t *)0);
    utb.modtime = time((time_t *)0) + 2678400;

    ret = utime("/yingtmp/test", &utb);
    if (ret < 0) {
        printf("utime failed, ret = %d\n", ret);
	} else { 
        printf("utime succeeded\n");
   }



#endif
#if 0
    utf.actime = times[0].tv_sec;
    utf.modtime = times[1].tv_sec;
    ret = utime("/yingtmp/test", &utf);
    if (ret < 0) {
        printf("utimes failed, ret = %d\n", ret);
	} else { 
        printf("utimes succeeded\n");
   }


#endif
#if 1
    times[0].tv_nsec = -2;
	if (utimensat(/*AT_FDCWD*/fd, "/yingtmp/test", times, 0) < 0) {
        printf("utimensat UTIME_OMIT failed\n");
	} else {
        printf("utimensat UTIME_OMIT succeeded\n");
    }
#endif 
#if 1
	times[0].tv_nsec = -1;
    if (utimensat(/*AT_FDCWD*/fd, "/yingtmp/test", times, 0) < 0) {
        printf("utimensat UTIME_NOW failed\n");
    } else {
        printf("YINGYING utimensat -2 succeeded\n");
    }
#endif    
    close(fd);
	return 0;
}
