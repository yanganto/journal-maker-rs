extern crate actix_web;
extern crate futures;
use actix_web::{
    http::Method,
    http,
    server,
    App,
    HttpRequest,
    HttpResponse,
    Result
};

fn index(_r: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body("Powered by Rust actix web"))
}

fn main() {
    server::new(
        || {
            vec![
//		//route.get("/auth")  POST, PUT
//		//route.get("/journal") GET, POST
//		//route.get("/journal/public") GET
//		//route.get("/journal/<user_name>/<journal>") GET
//		//route.get("/journal/<user_name>/<journal>") GET
//		//route.get("/image/<file_name>") POST

                App::new()
                    .resource( "/", |r| { r.method(Method::GET).f(index) })
            ]
        }
    )
    .bind("localhost:5000")
    .expect(&format!("Cannot bind the server to {}.", "localhost:5000"))
    .shutdown_timeout(30)
    .run();
}
