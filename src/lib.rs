pub mod exp;
pub mod fmt;
pub mod fnc;
pub mod fs;
pub mod obj;

/// Enable actix-web macros
#[cfg(feature = "actix-web")]
pub mod actix;

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
    fn clone() {
        let var: String = Default::default();

        assert_eq!(crate::obj::clone!(var), "");
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
