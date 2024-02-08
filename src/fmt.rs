#[doc(hidden)]
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
macro_rules! _lib_error {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use $crate::fmt::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        write!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_errorln {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use $crate::fmt::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        writeln!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_crash {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use $crate::fmt::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        write!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
        std::process::exit(1);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_crashln {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use $crate::fmt::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        writeln!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
        std::process::exit(1);
    }};
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
