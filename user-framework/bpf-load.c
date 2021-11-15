#include <stdio.h>
#include <stdarg.h>
#include <stdint.h>
#include <sys/mman.h>
#include <elf.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <linux/bpf.h>
#include <linux/perf_event.h>
#include <linux/hw_breakpoint.h>
#include <sys/ioctl.h>
#include <string.h>
#include <linux/perf_event.h>
#include <asm/unistd.h>
#include <linux/version.h>

#include "elf.h"

#define ERR(x...)                               \
    do {                                        \
        printf(x);                              \
        return 1;                               \
    } while(0);

#define PERR(x)                                 \
    do {                                        \
        perror(x);                              \
        return 1;                               \
    } while(0);

#define BPF_PROG_LOAD_DJW  0x1234beef
#define MAX_PROG_SZ (8192 * 4)


long stub_bpf_trace_printk(const char *fmt, uint32_t fmt_size, ...) {
    va_list args;
    va_start(args, fmt_size);
    vprintf(fmt, args);
    va_end(args);
    return 0;
}
uint64_t stub_bpf_get_current_pid_tgid(void) {
    return 0xdeadbeefdeadbeef;
}
void stub_bpf_test_call(void) {
    printf("Hello world!\n");
}

static long perf_event_open(struct perf_event_attr *hw_event, pid_t pid,
                            int cpu, int  group_fd, unsigned long flags)
{
    int ret;
    
    ret = syscall(__NR_perf_event_open, hw_event, pid, cpu,
                  group_fd, flags);
    return ret;
}

static int bpf(enum bpf_cmd cmd, union bpf_attr *attr, unsigned int size)
{
    return syscall(__NR_bpf, cmd, attr, size);
}

int do_actual_bpf(void *prog, size_t size) {
    union bpf_attr attr;
    memset(&attr, 0, sizeof(attr));
    attr.prog_type = BPF_PROG_TYPE_TRACEPOINT;
    strcpy(attr.prog_name,"handle_tp");
    attr.insn_cnt = size;
    attr.insns = (__u64)prog;
    attr.kern_version = KERNEL_VERSION(5, 13, 0);
    attr.license = (__u64)"GPL";
    return bpf(BPF_PROG_LOAD_DJW, &attr, sizeof(attr));
}

int main(int argc, char **argv) {
    int fd;
    void *area;
    size_t sz, n;
 
    if (argc != 2)
        ERR("Usage: %s <prog>\n", argv[0]);

    fd = open(argv[1], O_RDONLY);
    if (!fd)
        ERR("Couldn't open file %s\n", argv[1]);

    area = mmap(NULL, MAX_PROG_SZ,
                PROT_EXEC | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1, 0);
    if (area == MAP_FAILED)
        ERR("Couldn't create map of size %d\n", MAX_PROG_SZ);
    printf("Area is at %p\n", area);
    uint64_t entry = 0;
    size_t mem_size = MAX_PROG_SZ;
    if (elf_load(fd, argv[1], area, &mem_size, &entry))
        ERR("Couldn't load\n");
    printf("Entry point is 0x%lx\n", entry);
    //uint64_t (*run_prog)(void) = (uint64_t (*)(void))(area + entry);

    printf("Sending bpf %p (size %zu)\n", area + entry, mem_size);
    int bpf_fd = do_actual_bpf(area + entry, mem_size);
    printf("bpf_fd is %d\n", bpf_fd);
    if (bpf_fd <= 0) {
        PERR("Couldn't load BPF");
    }
    int fd2 = openat(AT_FDCWD, "/sys/kernel/debug/tracing/events/syscalls/sys_enter_write/id", O_RDONLY);

    char config_str[256];
    read(fd2, config_str, 256);
    close(fd2);

    struct perf_event_attr p_attr;
    memset(&p_attr, 0, sizeof(p_attr));
    p_attr.type = PERF_TYPE_TRACEPOINT;
    p_attr.size = PERF_ATTR_SIZE_VER5;
    p_attr.config = atoi(config_str);
    fd2 = perf_event_open(&p_attr, -1, 0, -1, PERF_FLAG_FD_CLOEXEC);
    ioctl(fd2, PERF_EVENT_IOC_SET_BPF, bpf_fd);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE, 0);

    printf("opening debug pipe");
    int fd3 = openat(AT_FDCWD, "/sys/kernel/debug/tracing/trace_pipe", O_RDONLY);
    for (;;) {
        char c;
        if (read(fd3, &c, 1) == 1)
            putchar(c);
    }

    return 0;
}

