use actix_web::{
    http::Method,
    http,
    server,
    App,
    HttpRequest,
    HttpResponse,
    Result,
    Path
};

fn index(_r: &HttpRequest) -> HttpResponse {
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body("Powered by Rust actix web")
}

pub fn handlers(base_url: &str) -> App {
    App::new()
        .prefix(base_url)
        .resource("/", |r| { 
        r.get().f(index) 
    })
}
