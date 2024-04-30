use crate::common::*;

use rex::linux::bpf::bpf_spin_lock;
use rex::map::*;
use rex::utils::*;

use rex::rex_map;
use rex::FieldTransmute;

#[repr(C)]
pub(crate) struct paxos_quorum {
    pub(crate) view: u32,
    pub(crate) opnum: u32,
    pub(crate) bitset: u32,
}

#[repr(C)]
pub(crate) struct paxos_ctr_state {
    pub(crate) state: ReplicaStatus,
    pub(crate) my_idx: u32,
    pub(crate) leader_idx: u32,
    pub(crate) batch_size: u32,
    pub(crate) view: u64,
    pub(crate) last_op: u64,
}

#[repr(C)]
pub(crate) struct paxos_batch {
    counter: u32,
    lock: bpf_spin_lock,
}

#[rex_map]
pub(crate) static MAP_CONFIGURE: IUArrayMap<PaxosConfigure> =
    IUArrayMap::new(FAST_REPLICA_MAX, 0);

#[rex_map]
pub(crate) static map_ctr_state: IUArrayMap<paxos_ctr_state> =
    IUArrayMap::new(1, 0);

#[rex_map]
pub(crate) static map_msg_last_op: IUArrayMap<u64> = IUArrayMap::new(1, 0);

#[rex_map]
pub(crate) static map_quorum: IUArrayMap<paxos_quorum> =
    IUArrayMap::new(QUORUM_BITSET_ENTRY, 0);

#[rex_map]
pub(crate) static batch_context: IUArrayMap<paxos_batch> =
    IUArrayMap::new(1, 0);

#[rex_map]
pub(crate) static map_prepare_buffer: IURingBuf = IURingBuf::new(1 << 20, 0);
#[rex_map]
pub(crate) static map_request_buffer: IURingBuf = IURingBuf::new(1 << 20, 0);

#[derive(FieldTransmute)]
#[repr(C, packed)]
pub struct eth_header {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}
