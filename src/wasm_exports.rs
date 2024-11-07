use std::alloc::Layout;

#[no_mangle]
unsafe fn alloc(size: usize) -> *mut u8 {
    let align = core::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    std::alloc::alloc(layout)
}

#[no_mangle]
unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let align = core::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    std::alloc::dealloc(ptr, layout);
}

extern "C" {
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

#[no_mangle]
extern "C" fn init_panic_hook() {
    std::panic::set_hook(Box::new(panic_hook));
}
