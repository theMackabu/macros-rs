#[doc(hidden)]
#[macro_export]
macro_rules! _lib_scaffold {
  (pub struct $name:ident { $($field:ident: $type:ty),* $(,)? }) => {
    impl $name {
      #[allow(dead_code, unused)]
      fn write(self, buf: &mut [u32]) {
        let mut offset = 0;
        $(
          let value = self.$field as u64;
          buf[offset] = value as u32;
          buf[offset + 1] = (value >> 32) as u32;
          #[allow(unused_assignments)]
          {
            offset += 2;
          }
        )*
      }
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct $name {
      $($field: $type),*
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_derive {
  (pub struct $name:ident { $($field:ident: $type:ty),* $(,)? }) => {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    struct $name {
      $($field: $type),*
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_clone {
    ($obj:expr) => {
        $obj.clone()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_lazy_lock {
    ($(#[$attr:meta])* static $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        $crate::__lazy_lock_internal!($(#[$attr])* () static $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub static $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        $crate::__lazy_lock_internal!($(#[$attr])* (pub) static $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        $crate::__lazy_lock_internal!($(#[$attr])* (pub ($($vis)+)) static $N : $T = $e; $($t)*);
    };
    () => ()
}

#[doc(hidden)]
#[macro_export]
macro_rules! __lazy_lock_internal {
    ($(#[$attr:meta])* ($($vis:tt)*) static $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        $(#[$attr])*
        $($vis)* static $N: std::sync::LazyLock<$T> = std::sync::LazyLock::new(|| $e);
        $crate::obj::lazy_lock!($($t)*);
    };
    () => ()
}

#[doc(inline)]
pub use _lib_scaffold as scaffold;

#[doc(inline)]
pub use _lib_derive as derive;

#[doc(inline)]
pub use _lib_clone as clone;

#[doc(inline)]
pub use _lib_lazy_lock as lazy_lock;
