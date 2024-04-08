use std::ffi::c_void;

use enum_dispatch::enum_dispatch;



use windows_sys::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, MEM_RELEASE, VirtualFreeEx};
use windows_sys::Win32::System::*;
use windows_sys::Win32::{Foundation::HANDLE};

#[enum_dispatch]
pub trait MemoryAllocator {
    fn alloc(
        &self,
        h_proc: HANDLE,
        start_address: Option<*const std::ffi::c_void>,
        size: u32,
    ) -> usize;
    fn free(&self, h_proc: HANDLE, dll_address: *const usize);
}

#[derive(PartialEq)]
#[enum_dispatch(MemoryAllocator)]
pub enum AllocatorMethod {
    VirtualAllocEx,
}

pub struct VirtualAllocEx;
impl MemoryAllocator for VirtualAllocEx {
    fn alloc(
        &self,
        h_proc: HANDLE,
        start_address: Option<*const std::ffi::c_void>,
        _size: u32,
    ) -> usize {
        unsafe {
            Memory::VirtualAllocEx(h_proc, start_address.unwrap_or(std::ptr::null()),  0x1000,
                MEM_COMMIT | MEM_RESERVE,  PAGE_READWRITE,
            ) as usize
        }
    }
    fn free(&self, h_proc: HANDLE, dll_address: *const usize) {
        unsafe { VirtualFreeEx(h_proc, dll_address as *mut c_void, 0x0, MEM_RELEASE)};
    }
}
