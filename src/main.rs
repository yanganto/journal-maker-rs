extern crate gotham;

use gotham::router::Router;
use gotham::router::builder::*;

mod handler;

fn router() -> Router {
        build_simple_router(|route| {
		route.get("/").to(handler::greeting);
		//route.get("/auth")  POST, PUT
		//route.get("/journal") GET, POST
		//route.get("/journal/public") GET
		//route.get("/journal/<user_name>/<journal>") GET
		//route.get("/image/<file_name>") POST
	})
}

pub fn main() {
    let addr = "127.0.0.1:9000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

