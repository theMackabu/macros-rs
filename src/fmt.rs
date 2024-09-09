#[doc(hidden)]
#[cfg(feature = "color")]
pub use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_string {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_str {
    ($x: expr) => {{
        let out: &'static str = { Box::leak($x.into_boxed_str()) };
        out
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_fmtstr {
    ($($t:tt)*) => {{
        let out: &'static str = { Box::leak(format!($($t)*).into_boxed_str()) };
        out
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _fmt_writer {
    ($func:ident, $handle:expr, crash($crash:expr), $($arg:tt)*) => {{
        use std::io::Write;
        let result = $func!(&mut $handle, $($arg)*);
        result.expect("Unable to write to handle (file handle closed?)");
        if $crash { std::process::exit(1) }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "color")]
macro_rules! _io_stderr {
    () => {{
        use $crate::fmt::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to set stderr color (file handle closed?)");
        stderr
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "color"))]
macro_rules! _io_stderr {
    () => {
        std::io::stderr()
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "color"))]
macro_rules! _lib_error {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(write, $crate::_io_stderr!(), crash(false), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "color"))]
macro_rules! _lib_errorln {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(writeln, $crate::_io_stderr!(), crash(false), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "color"))]
macro_rules! _lib_crash {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(write, $crate::_io_stderr!(), crash(true), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "color"))]
macro_rules! _lib_crashln {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(writeln, $crate::_io_stderr!(), crash(true), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "color")]
macro_rules! _lib_error {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(write, $crate::_io_stderr!(), crash(false), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "color")]
macro_rules! _lib_errorln {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(writeln, $crate::_io_stderr!(), crash(false), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "color")]
macro_rules! _lib_crash {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(write, $crate::_io_stderr!(), crash(true), $($arg)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "color")]
macro_rules! _lib_crashln {
    ($($arg:tt)*) => {
        $crate::_fmt_writer!(writeln, $crate::_io_stderr!(), crash(true), $($arg)*)
    };
}

#[doc(inline)]
pub use _lib_string as string;

#[doc(inline)]
pub use _lib_str as str;

#[doc(inline)]
pub use _lib_fmtstr as fmtstr;

#[doc(inline)]
pub use _lib_error as error;

#[doc(inline)]
pub use _lib_errorln as errorln;

#[doc(inline)]
pub use _lib_crash as crash;

#[doc(inline)]
pub use _lib_crashln as crashln;
