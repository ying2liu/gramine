#include <stdio.h>

#include <sys/time.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

int main(void) {
    struct stat statbuf;
    struct timespec times[2];
    int fd = open(/*"/home/yliu79/ying_repo/gramine/CI-Examples/utimensat/test"*/"test", O_RDWR, 0777);

    if (fd < 0) {
        printf("open file failed, fd=%d\n", fd);
    }
#if 1 
   	if (utimensat(AT_FDCWD, "test",NULL, 0) < 0) {
        printf("utimensat times = NULL, failed\n");
	} else {
        printf("YINGYING utimensat times = NULL pass\n");
    }
#endif
//	if (stat("test", &statbuf) < 0) {
  //      printf("stat failed\n");
   // }

//    times[0] = statbuf.st_atim;
//    times[1] = statbuf.st_mtim;
#if 0 
    times[0].tv_sec = 20;
    times[0].tv_nsec = 30;
    times[1].tv_sec = 40;
    times[1].tv_nsec = 50;
    printf("times[0].tv_sec=%ld, times[0].tv_nsec=%ld\n", times[0].tv_sec, times[0].tv_nsec);
    int ret = utimensat(AT_FDCWD, "test", times, 0);
    if (ret < 0) {
        printf("utimensat failed, ret = %d\n", ret);
	} else { 
        printf("pass\n");
   }
#endif
#if 0
	times[0].tv_nsec = -2;
	if (utimensat(AT_FDCWD, "test", times, 0) < 0) {
        printf("utimensat UTIME_OMIT failed\n");
	}
     
	times[0].tv_nsec = -1;
    if (utimensat(AT_FDCWD, "test", times, 0) < 0) {
        printf("utimensat UTIME_NOW failed\n");
    } else {
        printf("YINGYING utimensat -2 pass\n");
    }
  #endif
    close(fd);
	return 0;
}
