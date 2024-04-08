use log::*;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows_sys::Win32::System::Threading::{self, PROCESS_ALL_ACCESS};
use self::allocator::MemoryAllocator;
use self::executor::Executor;
use self::writer::MemoryWriter;
use std::ffi::CString;
use std::time::Duration;
pub mod allocator;
pub mod executor;
pub mod writer;

pub fn inject( allocator: &dyn MemoryAllocator, writer: &dyn MemoryWriter, executor: &dyn Executor, pid: u32, file_path: String,) {
        
    let file_path_cstring = CString::new(file_path).expect("Failed to create CString from file_path");
    info!("File path:{:?}", file_path_cstring);
    let h_proc: HANDLE = unsafe { Threading::OpenProcess(PROCESS_ALL_ACCESS, 0, pid) };
    info!("Target process handle -> {:#x}", h_proc);

    let dll_address = writer.write_memory(allocator,h_proc, file_path_cstring).unwrap();

    let thread_id = executor.execute(dll_address as usize, h_proc, allocator);
    info!("Thread ID-> {:?}", thread_id);

    std::thread::sleep(Duration::from_millis(1000));

    unsafe {
        allocator.free(h_proc, dll_address);
        CloseHandle(h_proc);
    }
}

pub fn get_load_library_address() -> unsafe extern "system" fn() -> isize {
    let h_k32: HANDLE = unsafe { GetModuleHandleA(b"kernel32.dll\0".as_ptr()) };
    debug!("kernel32.dll handle -> {:#x}", h_k32);
    let load_libary_a_thunk = match unsafe { GetProcAddress(h_k32, b"LoadLibraryA\0".as_ptr()) } {
        Some(x) => {
            info!("Found LoadLibaryA Thunk");
            x
        }
        _ => {
            error!("Failed to find LoadLibaryA");
            panic!("fuck");
        }
    };
    load_libary_a_thunk
}
