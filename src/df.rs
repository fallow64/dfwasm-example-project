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

pub fn print_string(s: &str) {
    for c in s.chars() {
        putc(c);
    }
}
