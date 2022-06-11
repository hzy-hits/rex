#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <unistd.h>

#include <linux/perf_event.h>
#include <linux/unistd.h>

#include "libiu.h"

#define EXE "./target/debug/" SAMPLE_NAME

#define SAMPLE_FREQ 3

static int pid;

static inline long perf_event_open(struct perf_event_attr *hw_event, pid_t pid,
							int cpu, int  group_fd, unsigned long flags)
{
	return syscall(__NR_perf_event_open, hw_event, pid, cpu, group_fd, flags);
}

int main(void)
{
	int base_fd, prog_fd, trace_id_fd, perf_event_fd, trace_pipe_fd;
	char config_str[256];
	struct perf_event_attr p_attr;
	int pmu_fd;

	// TODO: For now, only one out of the six events.
	struct perf_event_attr attr_type_sw = {
		.sample_freq = SAMPLE_FREQ,
		.freq = 1,
		.type = PERF_TYPE_SOFTWARE,
		.config = PERF_COUNT_SW_CPU_CLOCK,
	};

	struct perf_event_attr *attr = &attr_type_sw;

	iu_set_debug(1); // enable debug info

	base_fd = iu_prog_load(EXE);

	if (base_fd < 0)
		exit(1);

	prog_fd = iu_prog_get_subprog(base_fd, "iu_prog1");

	if (prog_fd < 0) {
		fprintf(stderr, "iu_prog1 not found\n");
		exit(1);
	}

	// trace_id_fd = openat(AT_FDCWD,
	// 	"/sys/kernel/debug/tracing/events/syscalls/sys_enter_dup/id", O_RDONLY);
	// if (trace_id_fd < 0) {
	// 	perror("openat(/sys/kernel/debug/tracing/events"
	// 		"/syscalls/sys_enter_dup/id)");
	// 	exit(1);
	// }
	// read(trace_id_fd, config_str, 256);
	// close(trace_id_fd);

	// memset(&p_attr, 0, sizeof(p_attr));
	// p_attr.type = PERF_TYPE_TRACEPOINT;
	// p_attr.size = PERF_ATTR_SIZE_VER5;
	// p_attr.config = atoi(config_str);
	// perf_event_fd = perf_event_open(&p_attr, -1, 0, -1, PERF_FLAG_FD_CLOEXEC);
	// if (perf_event_fd < 0) {
	// 	perror("perf_event_open");
	// 	exit(1);
	// }

	pid = fork();
	if (pid == 0) {
		trace_pipe_fd = openat(AT_FDCWD, "/sys/kernel/debug/tracing/trace_pipe",
			O_RDONLY);
		for (;;) {
			char c;
			if (read(trace_pipe_fd, &c, 1) == 1)
				putchar(c);
		}
		return 0;
	} else if (pid == -1) {
		printf("couldn't spawn process\n");
		return -1;
	}

	// TODO: For now, only _all_cpu tests (without _task tests)
	// TODO: For now, only the first CPU
	pmu_fd = perf_event_open(attr, -1, 0 /* cpu_idx */, -1, 0);

	ioctl(pmu_fd, PERF_EVENT_IOC_SET_BPF, prog_fd);
	ioctl(pmu_fd, PERF_EVENT_IOC_ENABLE, 0);

	system("dd if=/dev/zero of=/dev/null count=5000k status=none");

	return 0;
}
