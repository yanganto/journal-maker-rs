use serde_json;
use serde_derive;
use actix_web::{
    App,
    HttpRequest,
    HttpResponse
};


#[derive(Serialize, Deserialize)]
pub struct Account {
    user: String,
    password: String
}

fn login(req: &HttpRequest) -> HttpResponse {
    //TODO: Json decode the payload
    let account: Account = serde_json::from_str(r#"{"user": "yanganto", "password": "pw"}"#).unwrap();
    HttpResponse::Ok().into()
}

fn renew(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    print!("{:?}", req.match_info());
    HttpResponse::Ok().into()
}

pub fn handlers(base_url: &str) -> App {
    App::new()
        .prefix(base_url)
        .resource("/", |r| {
            r.post().f(login);
            r.put().f(renew);
        })
}
