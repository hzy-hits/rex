#![no_std]
#![no_main]

extern crate inner_unikernel_rt;
extern crate rlibc;

use inner_unikernel_rt::tracepoint::*;

fn iu_prog1_fn(obj: &tracepoint, ctx: &tp_ctx) -> u32 {
    let pid = (obj.bpf_get_current_pid_tgid() & 0xFFFFFFFF) as u32;
    obj.bpf_trace_printk("Rust triggered from PID %u.\n", pid as u64, 0, 0);
    return 0;
}

#[link_section = "tracepoint/syscalls/sys_enter_dup"]
static PROG: tracepoint = tracepoint::new(iu_prog1_fn, "iu_prog1", tp_ctx::Void);