#[doc(hidden)]
#[macro_export]
macro_rules! _lib_inc {
    ($id:expr) => {{
        let _rv = $id;
        $id += 1;
        _rv
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_dec {
    ($id:expr) => {{
        let _rv = $id;
        $id -= 1;
        _rv
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_then {
    ($c:expr, $v:expr) => {
        if $c {
            $v;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_try {
    ($expr:expr, $err_handler:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                let handler: fn(_) = $err_handler;
                handler(err);
                return Err(err.into());
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_attempt {
   (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
      if let Err ($e) = $a $b
   };
   (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      $crate::_lib_attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
   };
   ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      $crate::_lib_attempt!{@recurse ($e) { $($tail)* } $($handler)* }
   };
}

#[doc(inline)]
pub use _lib_inc as inc;

#[doc(inline)]
pub use _lib_dec as dec;

#[doc(inline)]
pub use _lib_ternary as ternary;

#[doc(inline)]
pub use _lib_then as then;

#[doc(inline)]
pub use _lib_try as unwrap;

#[doc(inline)]
pub use _lib_attempt as attempt;
