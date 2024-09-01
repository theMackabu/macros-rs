#[doc(hidden)]
#[macro_export]
macro_rules! _os_env_set_sync {
    ( $( $key:ident = $val:expr ),* $(,)? ) => {{
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _g = LOCK.lock().unwrap();

        $(
            // SAFETY: the lock ensures this function blocks until the lock guard is released
            unsafe { std::env::set_var(stringify!($key), $val); }
        )*
    }};
}

#[doc(inline)]
pub use _os_env_set_sync as set_env_sync;
