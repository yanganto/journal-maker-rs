extern crate actix_web;
extern crate futures;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod api;
mod files;

use actix_web::server;

fn main() {
    server::new(
        || {
            vec![
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
