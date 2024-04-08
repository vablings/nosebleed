use enum_dispatch::enum_dispatch;
use log::info;
use windows_sys::Win32::{Foundation::HANDLE, System::Threading};

#[enum_dispatch]
pub trait Executor {
    fn execute(&self, remote_address: usize, handle: HANDLE, allocator: &dyn super::allocator::MemoryAllocator) -> u32;
}
#[derive(PartialEq)]
#[enum_dispatch(Executor)]
pub enum ExecutorMethod {
    CreateRemoteThread,
    ThreadHijacking,
}
pub struct CreateRemoteThread;
impl Executor for CreateRemoteThread {
    fn execute(&self, remote_address: usize, handle: HANDLE, _allocator: &dyn super::allocator::MemoryAllocator) -> u32 {
        info!("CreateRemoteThread Executor -> {}", remote_address);
        let mut thread_id = 0;
        unsafe {
            Threading::CreateRemoteThread(
                handle,
                core::ptr::null(),
                0,
                Some(core::mem::transmute(super::get_load_library_address())),
                remote_address as _,
                0,
                &mut thread_id,
            );
        }
        thread_id
    }
}
pub struct ThreadHijacking;
impl Executor for ThreadHijacking {
    fn execute(&self, _remote_address: usize, _handle: HANDLE, _allocator: &dyn super::allocator::MemoryAllocator) -> u32 {
        todo!("");
    }
}
