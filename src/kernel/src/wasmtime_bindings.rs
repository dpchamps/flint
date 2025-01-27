use core::{ptr};

// There's a lot more that needs to be implemented here
// But this is the bare minimum to get a build.

static mut WASMTIME_TLS: *mut u8 = ptr::null_mut();

#[no_mangle]
pub extern "C" fn wasmtime_tls_get() -> *mut u8 {
    unsafe { WASMTIME_TLS }
}

#[no_mangle]
pub extern "C" fn wasmtime_tls_set(ptr: *mut u8){
    unsafe {
        WASMTIME_TLS = ptr
    }
}


#[no_mangle]
pub extern "C" fn wasmtime_longjmp(jmp_buf_ptr: *const u8) {
    unsafe {
        // I'm maybe 70% confident this is correct.
        // https://stackoverflow.com/questions/77142453/how-to-handle-longjmp-case-of-c-in-rust
        // If we're stack unwinding, let's just panic. What's the worst that could happen.
        panic!("{:?}", jmp_buf_ptr)
    }
}

// This is quite awful. Dirty Dirty hacks.
#[no_mangle]
pub extern "C" fn wasmtime_setjmp(
    jmp_buf_out: *mut *const u8,
    callback: extern "C" fn(*mut u8, *mut u8) -> bool,
    payload: *mut u8,
    callee: *mut u8,
) -> bool {
    return callback(payload, callee);
}