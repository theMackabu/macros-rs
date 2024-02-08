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

#[doc(inline)]
pub use _lib_scaffold as scaffold;

#[doc(inline)]
pub use _lib_derive as derive;

#[doc(inline)]
pub use _lib_clone as clone;
