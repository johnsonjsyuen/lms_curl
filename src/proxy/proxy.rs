#[macro_use]
extern crate log;
extern crate env_logger;
extern crate lib_lms_curl;

use lib_lms_curl::curl_api;

use futures::IntoFuture;

use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};

#[get("/{proxy_path}")]
fn proxy(req: HttpRequest, proxy_path: web::Path<String>) -> String {
    println!("REQ: {:?}", req);

    curl_api(proxy_path.as_str())

}

#[get("/")]
fn no_params() -> &'static str {
    "Welcome to LMS Proxy!\r\n\n\n\n
    All requests here will be proxied to the LMS Dev API, automatically handling JWT refresh\n"
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(no_params)
            .service(proxy)
    })
        .bind("127.0.0.1:8080")?
        .run()
}
