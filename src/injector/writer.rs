use std::{error::Error, ffi::CString};

use enum_dispatch::enum_dispatch;
use log::*;
use windows_sys::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug,
    },
};

#[enum_dispatch]
pub trait MemoryWriter {
    fn write_memory(
        &self,
        allocator: &dyn super::allocator::MemoryAllocator,
        h_proc: HANDLE,
        file_path: CString,
    ) -> Result<*const usize, Box<dyn Error>>;
}

#[derive(PartialEq)]
#[enum_dispatch(MemoryWriter)]
pub enum MemoryWriterMethod {
    LoadLibary,
    ManualMap,
}

pub struct LoadLibary;
impl MemoryWriter for LoadLibary {
    fn write_memory(
        &self,
        allocator: &dyn super::allocator::MemoryAllocator,
        h_proc: HANDLE,
        file_path: CString,
) -> Result<*const usize, Box<dyn Error>> {


        let remote_dll_name_address = allocator.alloc(h_proc, None, 0x1000);
        info!(
            "Target process file_pathlocation -> {:#x}",
            remote_dll_name_address
        );
        let mut dll_name_buffer = [0x0u8; 0x1000];

        for (i, b) in file_path.as_bytes().iter().enumerate() {
            dll_name_buffer[i] = *b;
        }



        let mut _bytes_written = 0;
        unsafe {
            Debug::WriteProcessMemory(
                h_proc,
                remote_dll_name_address as _,
                dll_name_buffer.as_ptr() as _,
                0x1000,
                &mut _bytes_written,
            );
        };

        info!("dll_path location -> {:#x}", remote_dll_name_address);

        Ok(remote_dll_name_address as *const usize)
    }
}

pub struct ManualMap;
impl MemoryWriter for ManualMap {
    fn write_memory(
        &self,
        _allocator: &dyn super::allocator::MemoryAllocator,
        _h_proc: HANDLE,
        _file_path: CString,
    ) -> Result<*const usize, Box<dyn Error>> {
        Ok(0 as *const usize)
        //todo!();
    }
}
