use std::alloc::Layout;

#[unsafe(no_mangle)]
unsafe fn alloc(size: usize) -> *mut u8 {
    let align = core::mem::align_of::<usize>();
    let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
    unsafe { std::alloc::alloc(layout) }
}

#[unsafe(no_mangle)]
unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let align = core::mem::align_of::<usize>();
    let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
    unsafe {
        std::alloc::dealloc(ptr, layout);
    }
}

unsafe extern "C" {
    fn error(msg: *mut u8, msg_len: usize);
}

pub fn fail(msg: impl Into<String>) {
    let msg = msg.into();
    let len = msg.len();
    let msg = Box::into_raw(msg.into_boxed_str());
    unsafe {
        error(msg as *mut u8, len);
    }
}

fn panic_hook(info: &std::panic::PanicHookInfo) {
    let msg = info.to_string();
    fail(&msg);
}

#[unsafe(no_mangle)]
extern "C" fn init_panic_hook() {
    std::panic::set_hook(Box::new(panic_hook));
}
