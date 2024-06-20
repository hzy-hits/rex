#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

extern crate rex;

use rex::bpf_printk;
use rex::linux::bpf::*;
use rex::linux::perf_event::PERF_MAX_STACK_DEPTH;
use rex::map::*;
use rex::perf_event::*;
use rex::{entry_link, rex_map, Result};

pub const TASK_COMM_LEN: usize = 16;

// What if user does not use repr(C)?
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyT {
    pub comm: [i8; TASK_COMM_LEN],
    pub kernstack: u32,
    pub userstack: u32,
}

#[rex_map]
static counts: RexHashMap<KeyT, u64> = RexHashMap::new(10000, 0);

#[rex_map]
static stackmap: RexStackMap<u32, [u64; PERF_MAX_STACK_DEPTH as usize]> =
    RexStackMap::new(10000, 0);

pub const KERN_STACKID_FLAGS: u64 = BPF_F_FAST_STACK_CMP as u64;
pub const USER_STACKID_FLAGS: u64 =
    (BPF_F_FAST_STACK_CMP | BPF_F_USER_STACK) as u64;

#[inline(always)]
fn iu_prog1_fn(obj: &perf_event, ctx: &bpf_perf_event_data) -> Result {
    let cpu: u32 = obj.bpf_get_smp_processor_id();
    let mut value_buf: bpf_perf_event_value = bpf_perf_event_value {
        counter: 0,
        enabled: 0,
        running: 0,
    };
    let mut key: KeyT = KeyT {
        comm: [0; TASK_COMM_LEN],
        kernstack: 0,
        userstack: 0,
    };
    if ctx.sample_period() < 10000 {
        return Ok(0);
    }

    obj.bpf_get_current_task()
        .map(|t| {
            t.get_comm(&mut key.comm);
            0u64
        })
        .ok_or_else(|| 0i32)?;

    key.kernstack = obj
        .bpf_get_stackid_pe(ctx, &stackmap, KERN_STACKID_FLAGS)
        .map_err(|_| {
        bpf_printk!(
            obj,
            "CPU-%d period %lld ip %llx",
            cpu as u64,
            ctx.sample_period(),
            ctx.rip()
        );
        0i32
    })? as u32;

    key.userstack = obj
        .bpf_get_stackid_pe(ctx, &stackmap, USER_STACKID_FLAGS)
        .map_err(|_| {
        bpf_printk!(
            obj,
            "CPU-%d period %lld ip %llx",
            cpu as u64,
            ctx.sample_period(),
            ctx.rip()
        );
        0i32
    })? as u32;

    obj.bpf_perf_prog_read_value(ctx, &mut value_buf)
        .map_or_else(
            |err| {
                bpf_printk!(obj, "Get Time Failed, ErrCode: %d", err as u64);
                err as u64
            },
            |val| {
                bpf_printk!(
                    obj,
                    "Time Enabled: %llu, Time Running: %llu",
                    value_buf.enabled,
                    value_buf.running
                );
                val as u64
            },
        );

    if ctx.addr() != 0 {
        bpf_printk!(obj, "Address recorded on event: %llx", ctx.addr());
    }

    match obj.bpf_map_lookup_elem(&counts, &key) {
        None => {
            obj.bpf_map_update_elem(&counts, &key, &1, BPF_NOEXIST as u64)?;
        }
        Some(val) => {
            *val += 1;
        }
    }
    Ok(0)
}

#[entry_link(inner_unikernel/perf_event)]
static PROG: perf_event = perf_event::new(iu_prog1_fn, "iu_prog1");
