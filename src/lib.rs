pub mod exp;
pub mod fmt;
pub mod fnc;
pub mod fs;
pub mod obj;
pub mod os;

/// Enable actix-web macros
#[cfg(feature = "actix-web")]
pub mod actix;

/// Enable everything
pub mod everything {
    pub use super::exp::*;
    pub use super::fmt::*;
    pub use super::fnc::*;
    pub use super::fs::*;
    pub use super::obj::*;
    pub use super::os::*;

    #[cfg(feature = "actix-web")]
    pub use super::actix::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn inc() {
        let mut num = 3;

        crate::exp::inc!(num);
        assert_eq!(num, 4);

        crate::exp::inc!(num);
        assert_eq!(num, 5);
    }

    #[test]
    fn dec() {
        let mut num = 3;

        crate::exp::dec!(num);
        assert_eq!(num, 2);

        crate::exp::dec!(num);
        assert_eq!(num, 1);
    }

    #[test]
    fn ternary() {
        let test = |val: bool| crate::exp::ternary!(val, "something", "nothing");

        assert_eq!(test(true), "something");
        assert_eq!(test(false), "nothing");
    }

    #[test]
    fn then() {
        let mut var = "";

        crate::exp::then!(true, var = "something");
        assert_eq!(var, "something");
    }

    #[test]
    fn string() {
        assert_eq!(crate::fmt::string!(), "");
        assert_eq!(crate::fmt::string!("something"), "something".to_string());
    }

    #[test]
    fn str() {
        assert_eq!(crate::fmt::str!("something".to_string()), "something");
    }

    #[test]
    fn set_env() {
        let mut rng = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;

        let test_value = std::iter::repeat_with(|| {
            rng = rng.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1 << 48) - 1);
            let byte = (rng >> 24) as u8;

            match byte % 3 {
                0 => (b'A' + (byte % 26)) as char,
                1 => (b'a' + (byte % 26)) as char,
                _ => (b'0' + (byte % 10)) as char,
            }
        })
        .take(16)
        .collect::<String>();

        crate::os::set_env_sync!(MACROS_ENV_1 = test_value.clone());
        assert_eq!(std::env::var("MACROS_ENV_1"), Ok(test_value));
    }

    #[test]
    fn fmtstr() {
        let data = 123;
        let var = "hello";

        assert_eq!(crate::fmt::fmtstr!("{var} world {data}{}", 45), "hello world 12345");
    }

    #[test]
    fn fn_name() {
        assert_eq!(crate::fnc::function_name!(), "fn_name");
    }

    #[test]
    fn fn_path() {
        assert_eq!(crate::fnc::function_path!(), "fn_path");
    }

    #[test]
    fn crash_test_facade() {
        use colored::Colorize;
        use std::io::Write;

        write!(&mut std::io::stdout(), "-------------------------\n").unwrap();
        crate::fmt::error!("error! {} {}", "print-test".bright_white(), "ok\n".green());
        crate::fmt::errorln!("errorln! {} {}", "print-test".bright_white(), "ok".green());
        write!(&mut std::io::stdout(), "-------------------------\n\n").unwrap();
    }

    #[test]
    fn clone() {
        let var: String = Default::default();

        assert_eq!(crate::obj::clone!(var), "");
    }

    #[test]
    fn lazy_lock() {
        crate::obj::lazy_lock! {
            static TEST: String = Default::default();
        }

        assert_eq!(&*TEST, "");
    }

    #[test]
    fn file_exists() { assert_eq!(crate::fs::file_exists!("tests/file.txt"), true) }

    #[test]
    fn folder_exists() { assert_eq!(crate::fs::folder_exists!("tests/dir"), true) }

    #[test]
    fn path_empty() { assert_eq!(crate::fs::path_empty!("tests/empty_file.txt"), true) }

    #[test]
    fn path_content() { assert_eq!(crate::fs::path_empty!("tests/file.txt"), false) }

    #[actix_web::test]
    #[cfg(feature = "actix-web")]
    async fn actix_web_html() {
        use actix_http::{HttpService, Request};
        use actix_http_test::test_server;
        use actix_utils::future::ok;
        use actix_web::http::header::HeaderValue;
        use std::convert::Infallible;

        let mut srv = test_server(|| {
            HttpService::build()
                .h1(|req: Request| {
                    assert!(req.peer_addr().is_some());
                    ok::<_, Infallible>(crate::actix::send_html!().finish())
                })
                .tcp()
        })
        .await;

        let response = srv.get("/").send().await.unwrap();
        assert_eq!(response.headers().get("Content-Type"), Some(&HeaderValue::from_static("text/html; charset=utf-8")));

        srv.stop().await;
    }

    #[actix_web::test]
    #[cfg(feature = "actix-web")]
    async fn actix_web_ok() {
        use actix_http::{HttpService, Request};
        use actix_http_test::test_server;
        use actix_utils::future::ok;
        use std::convert::Infallible;

        let mut srv = test_server(|| {
            HttpService::build()
                .h1(|req: Request| {
                    assert!(req.peer_addr().is_some());
                    ok::<_, Infallible>(crate::actix::ok!().finish())
                })
                .tcp()
        })
        .await;

        let response = srv.get("/").send().await.unwrap();
        assert!(response.status().is_success());

        srv.stop().await;
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! cfg_if {
     (
          $(
                if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
          ) else+
          else { $( $e_tokens:tt )* }
     ) => {
          $crate::cfg_if! {
                @__items () ;
                $(
                     (( $i_meta ) ( $( $i_tokens )* )) ,
                )+
                (() ( $( $e_tokens )* )) ,
          }
     };
     (
          if #[cfg( $i_meta:meta )] { $( $i_tokens:tt )* }
          $(
                else if #[cfg( $e_meta:meta )] { $( $e_tokens:tt )* }
          )*
     ) => {
          $crate::cfg_if! {
                @__items () ;
                (( $i_meta ) ( $( $i_tokens )* )) ,
                $(
                     (( $e_meta ) ( $( $e_tokens )* )) ,
                )*
          }
     };
     (@__items ( $( $_:meta , )* ) ; ) => {};
     (
          @__items ( $( $no:meta , )* ) ;
          (( $( $yes:meta )? ) ( $( $tokens:tt )* )) ,
          $( $rest:tt , )*
     ) => {
          #[cfg(all(
                $( $yes , )?
                not(any( $( $no ),* ))
          ))]
          $crate::cfg_if! { @__identity $( $tokens )* }
          $crate::cfg_if! {
                @__items ( $( $no , )* $( $yes , )? ) ;
                $( $rest , )*
          }
     };
     (@__identity $( $tokens:tt )* ) => {
          $( $tokens )*
     };
}
