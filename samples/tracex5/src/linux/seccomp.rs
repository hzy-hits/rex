/* automatically generated by rust-bindgen 0.59.2 */

pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const SECCOMP_MODE_DISABLED: u32 = 0;
pub const SECCOMP_MODE_STRICT: u32 = 1;
pub const SECCOMP_MODE_FILTER: u32 = 2;
pub const SECCOMP_SET_MODE_STRICT: u32 = 0;
pub const SECCOMP_SET_MODE_FILTER: u32 = 1;
pub const SECCOMP_GET_ACTION_AVAIL: u32 = 2;
pub const SECCOMP_GET_NOTIF_SIZES: u32 = 3;
pub const SECCOMP_FILTER_FLAG_TSYNC: u32 = 1;
pub const SECCOMP_FILTER_FLAG_LOG: u32 = 2;
pub const SECCOMP_FILTER_FLAG_SPEC_ALLOW: u32 = 4;
pub const SECCOMP_FILTER_FLAG_NEW_LISTENER: u32 = 8;
pub const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: u32 = 16;
pub const SECCOMP_RET_KILL_PROCESS: u32 = 2147483648;
pub const SECCOMP_RET_KILL_THREAD: u32 = 0;
pub const SECCOMP_RET_KILL: u32 = 0;
pub const SECCOMP_RET_TRAP: u32 = 196608;
pub const SECCOMP_RET_ERRNO: u32 = 327680;
pub const SECCOMP_RET_USER_NOTIF: u32 = 2143289344;
pub const SECCOMP_RET_TRACE: u32 = 2146435072;
pub const SECCOMP_RET_LOG: u32 = 2147221504;
pub const SECCOMP_RET_ALLOW: u32 = 2147418112;
pub const SECCOMP_RET_ACTION_FULL: u32 = 4294901760;
pub const SECCOMP_RET_ACTION: u32 = 2147418112;
pub const SECCOMP_RET_DATA: u32 = 65535;
pub const SECCOMP_USER_NOTIF_FLAG_CONTINUE: u32 = 1;
pub const SECCOMP_ADDFD_FLAG_SETFD: u32 = 1;
pub const SECCOMP_ADDFD_FLAG_SEND: u32 = 2;
pub const SECCOMP_IOC_MAGIC: u8 = 33u8;
pub type __s8 = i8;
pub type __u8 = u8;
pub type __s16 = i16;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __s64 = i64;
pub type __u64 = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [u64; 16usize],
}
pub type __kernel_sighandler_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: i32)>;
pub type __kernel_key_t = i32;
pub type __kernel_mqd_t = i32;
pub type __kernel_old_uid_t = u16;
pub type __kernel_old_gid_t = u16;
pub type __kernel_old_dev_t = u64;
pub type __kernel_long_t = i64;
pub type __kernel_ulong_t = u64;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = u32;
pub type __kernel_pid_t = i32;
pub type __kernel_ipc_pid_t = i32;
pub type __kernel_uid_t = u32;
pub type __kernel_gid_t = u32;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = i32;
pub type __kernel_uid32_t = u32;
pub type __kernel_gid32_t = u32;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [i32; 2usize],
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = i64;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = i64;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = i32;
pub type __kernel_clockid_t = i32;
pub type __kernel_caddr_t = *mut i8;
pub type __kernel_uid16_t = u16;
pub type __kernel_gid16_t = u16;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_data {
    pub nr: i32,
    pub arch: __u32,
    pub instruction_pointer: __u64,
    pub args: [__u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif_sizes {
    pub seccomp_notif: __u16,
    pub seccomp_notif_resp: __u16,
    pub seccomp_data: __u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif {
    pub id: __u64,
    pub pid: __u32,
    pub flags: __u32,
    pub data: seccomp_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif_resp {
    pub id: __u64,
    pub val: __s64,
    pub error: __s32,
    pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif_addfd {
    pub id: __u64,
    pub flags: __u32,
    pub srcfd: __u32,
    pub newfd: __u32,
    pub newfd_flags: __u32,
}
