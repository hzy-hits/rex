#![no_std]
#![no_main]

extern crate inner_unikernel_rt;

use inner_unikernel_rt::linux::bpf::*;
use inner_unikernel_rt::map::IUMap;
use inner_unikernel_rt::tracepoint::*;
use inner_unikernel_rt::{bpf_printk, entry_link, Result, MAP_DEF};

MAP_DEF!(map_hash, u32, u64, BPF_MAP_TYPE_HASH, 1, 0);
MAP_DEF!(map_array, u32, u64, BPF_MAP_TYPE_ARRAY, 1, 0);

fn iu_prog1_fn(obj: &tracepoint, _: tp_ctx) -> Result {
    let zero = 0u32;

    let random = obj.bpf_get_prandom_u32() as u64;
    obj.bpf_map_update_elem(&map_hash, &zero, &random, BPF_ANY as u64)?;

    let start = obj.bpf_ktime_get_ns();
    obj.bpf_map_lookup_elem(&map_hash, &zero);
    let end = obj.bpf_ktime_get_ns();

    bpf_printk!(obj, "Time elapsed: %llu", end - start);

    // let random = obj.bpf_get_prandom_u32() as u64;
    // obj.bpf_map_update_elem(&map_array, &zero, &random, BPF_ANY as u64)?;
    //
    // let start = obj.bpf_ktime_get_ns();
    // obj.bpf_map_lookup_elem(&map_array, &zero);
    // let end = obj.bpf_ktime_get_ns();
    //
    // bpf_printk!(obj, "Time elapsed: %llu", end - start);

    Ok(0)
}

#[entry_link(inner_unikernel/tracepoint/syscalls/sys_enter_getcwd)]
static PROG: tracepoint = tracepoint::new(iu_prog1_fn, "iu_prog1", tp_type::Void);
