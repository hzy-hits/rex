#![no_std]
#![no_main]

extern crate inner_unikernel_rt;
extern crate rlibc;

use inner_unikernel_rt::bpf_printk;
use inner_unikernel_rt::linux::bpf::*;
use inner_unikernel_rt::tracepoint::*;

fn iu_prog1_fn(obj: &tracepoint, ctx: &tp_ctx) -> u32 {
    let option_task = obj.bpf_get_current_task();

    let time = obj.bpf_ktime_get_ns();
    bpf_printk!(obj, "Time: %llu\n", time);
    let origin_time = obj.bpf_ktime_get_boot_ns_origin();
    bpf_printk!(obj, "Origin Time: %llu\n", origin_time);
    assert!(origin_time - time < u64::MAX / 10000);

    let boot_time = obj.bpf_ktime_get_boot_ns();
    bpf_printk!(obj, "Boot Time: %llu\n", boot_time);
    let origin_boot_time = obj.bpf_ktime_get_boot_ns_origin();
    bpf_printk!(obj, "Origin Boot Time: %llu\n", origin_boot_time);
    assert!(origin_boot_time - boot_time < u64::MAX / 10000);

    0
}

#[link_section = "tracepoint/syscalls/sys_enter_dup"]
static PROG: tracepoint = tracepoint::new(iu_prog1_fn, "iu_prog1", tp_ctx::Void);
