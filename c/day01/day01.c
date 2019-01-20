#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    int fd = open(argv[1], O_RDONLY);
    char buf[8192];
    size_t nbytes = sizeof(buf);
    ssize_t bytes_read;

    int flr = 0;
    int p2 = -1;
    int i = 1;
    while((bytes_read = read(fd, buf, nbytes)) > 0) {
        for (int j = 0; j < bytes_read; j++, i++) {
            char ch = buf[j];
            switch (ch) {
                case '(':
                    flr++;
                    break;
                case ')':
                    flr--;
            }
            if (flr == -1 && p2 == -1) {
                p2 = i;
            }
        }
    }

    printf("Part1: %d\n", flr);
    printf("Part2: %d\n", p2);

    close(fd);
    return 0;
}