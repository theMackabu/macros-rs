#[doc(hidden)]
#[macro_export]
macro_rules! _lib_send_html {
    () => {
        actix_web::HttpResponse::build(actix_web::http::StatusCode::OK).content_type(actix_web::http::header::ContentType::html())
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_ok {
    () => {
        actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
    };
}

#[doc(inline)]
pub use _lib_send_html as send_html;

#[doc(inline)]
pub use _lib_ok as ok;
