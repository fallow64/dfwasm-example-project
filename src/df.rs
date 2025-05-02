extern crate alloc;

mod imports {
    #[link(wasm_import_module = "env")]
    unsafe extern "C" {
        #[link_name = "putc"]
        pub unsafe fn putc(c: i32);

        #[link_name = "wait"]
        pub unsafe fn wait(n: i32);

        #[link_name = "millis"]
        pub unsafe fn millis() -> i32;

        #[link_name = "putstr"]
        pub unsafe fn putstr(s: *const u8, len: i32);
    }
}

pub fn putc(c: char) {
    unsafe { imports::putc(c as i32) }
}

pub fn wait(n: i32) {
    unsafe { imports::wait(n) }
}

pub fn millis() -> i32 {
    unsafe { imports::millis() }
}

pub fn putstr(s: &str) {
    let len = s.len() as i32;
    let ptr = s.as_ptr() as *const u8;
    unsafe { imports::putstr(ptr, len) }
}
