extern crate actix_web;
extern crate futures;

mod api;
mod files;

use actix_web::server;

fn main() {
    server::new(
        || {
            vec![
//		//route.get("/image/<file_name>") POST
                api::auth::handlers("/auth"),
                api::image::handlers("/image"),
                api::journal::handlers("/journal"),
                files::handlers("/")
            ]
        }
    )
    .bind("localhost:5000")
    .expect(&format!("Cannot bind the server to {}.", "localhost:5000"))
    .shutdown_timeout(30)
    .run();
}
