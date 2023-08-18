use core::panic::PanicInfo;

use crate::per_cpu::this_cpu_ptr_mut;
use crate::stub;

const ENTRIES_SIZE: usize = 64;

pub(crate) type CleanupFn = fn(*const ()) -> ();

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub(crate) struct CleanupEntry {
    pub(crate) valid: u64,
    pub(crate) cleanup_fn: Option<CleanupFn>,
    pub(crate) cleanup_arg: *const (),
}

impl CleanupEntry {
    pub fn new(cleanup_fn: CleanupFn, cleanup_arg: *const ()) -> Self {
        Self {
            valid: 1,
            cleanup_fn: Some(cleanup_fn),
            cleanup_arg,
        }
    }

    pub fn cleanup(&self) {
        if self.valid != 0 {
            if let Some(cleanup_fn) = self.cleanup_fn {
                (cleanup_fn)(self.cleanup_arg);
            }
        }
    }
}

impl Default for CleanupEntry {
    fn default() -> Self {
        Self {
            valid: 0,
            cleanup_fn: None,
            cleanup_arg: core::ptr::null(),
        }
    }
}

pub(crate) struct CleanupEntries<'a> {
    entries: &'a mut [CleanupEntry],
}

impl<'a> CleanupEntries<'a> {
    #[inline]
    pub(crate) fn cleanup_entries_this_cpu() -> CleanupEntries<'a> {
        let entries: &mut [CleanupEntry];
        unsafe {
            let entries_ptr: *mut CleanupEntry =
                this_cpu_ptr_mut(stub::iu_cleanup_entries_addr());
            entries =
                core::slice::from_raw_parts_mut(entries_ptr, ENTRIES_SIZE);
        }
        Self { entries }
    }

    /// This function is called by object constructors
    /// Panic is allowed here
    pub(crate) fn find_next_emtpy_entry(
        &mut self,
    ) -> (usize, &mut CleanupEntry) {
        for (idx, entry) in self.entries.iter_mut().enumerate() {
            if entry.valid == 0 {
                return (idx, entry);
            }
        }
        panic!("Object count exceeded\n");
    }

    /// This function is called on panic to cleanup everything
    /// It **must** not cause another panic
    pub(crate) fn cleanup_all(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.cleanup();
            *entry = Default::default();
        }
    }
}

impl core::ops::Index<usize> for CleanupEntries<'_> {
    type Output = CleanupEntry;

    #[inline(always)]
    fn index(&self, index: usize) -> &CleanupEntry {
        &self.entries[index]
    }
}

impl core::ops::IndexMut<usize> for CleanupEntries<'_> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut CleanupEntry {
        &mut self.entries[index]
    }
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CleanupEntries::cleanup_entries_this_cpu().cleanup_all();

    // Print the msg
    let mut msg = [0u8; 128];
    if let Some(args) = info.message() {
        // Only works in the most trivial case: no format args
        if let Some(s) = args.as_str() {
            let len = core::cmp::min(msg.len() - 1, s.len());
            msg[..len].copy_from_slice(&(*s).as_bytes()[..len]);
            msg[len] = 0u8;
        } else {
            let s = "Rust program panicked\n\0";
            msg[..s.len()].copy_from_slice(s.as_bytes());
        }
    } else if let Some(s) = info.payload().downcast_ref::<&str>() {
        let len = core::cmp::min(msg.len() - 1, s.len());
        msg[..len].copy_from_slice(&(*s).as_bytes()[..len]);
        msg[len] = 0u8;
    } else {
        let s = "Rust program panicked\n\0";
        msg[..s.len()].copy_from_slice(s.as_bytes());
    }

    unsafe {
        let landingpad: unsafe extern "C" fn(*const u8) -> ! =
            core::mem::transmute(stub::iu_landingpad_addr());
        landingpad(msg.as_ptr())
    }
}
