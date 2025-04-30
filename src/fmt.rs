pub trait DfFormat {
    fn df_format_write(&self, s: &mut alloc::string::String) -> Result<(), &'static str>;
}

impl DfFormat for char {
    fn df_format_write(&self, s: &mut alloc::string::String) -> Result<(), &'static str> {
        s.push(*self);
        Ok(())
    }
}

impl DfFormat for &str {
    fn df_format_write(&self, s: &mut alloc::string::String) -> Result<(), &'static str> {
        s.push_str(self);
        Ok(())
    }
}

macro_rules! impl_number {
    ($t:ty) => {
        impl DfFormat for $t {
            fn df_format_write(&self, s: &mut alloc::string::String) -> Result<(), &'static str> {
                let mut buffer = itoa::Buffer::new();
                let res = buffer.format(*self);
                s.push_str(res);
                Ok(())
            }
        }
    };
}

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(usize);
impl_number!(isize);
impl_number!(u128);
impl_number!(i128);

#[macro_export]
macro_rules! print {
    ($($arg:expr),* $(,)?) => {{
        use $crate::fmt::DfFormat;
        let mut output = alloc::string::String::new();
        $(
            $arg.df_format_write(&mut output).unwrap();
        )*
        $crate::df::print_string(output.as_str());
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::df::putc('\n');
    };
}
