use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use serde_json;
use serde_derive;
use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    Result,
    error,
    Error,
    Json
};
use jwt::{encode, Header};
use chrono::Utc;
use sha2::Digest;



#[derive(Deserialize)]
struct Account {
    user: String,
    password: String
}

#[derive(Serialize)]
struct Token {
    token: String
}

#[derive(Serialize, Deserialize)]
struct Claim {
    user: String,
    exp: usize
}

fn login(account: Json<Account>) -> Result<Json<Token>> {
    let claim = Claim {
        user: account.user.clone(),
        exp: Utc::now().timestamp() as usize + 300usize
    };
    let auth = match account.password.as_ref() {
        "LittleFoot" => {
            if Path::new(&format!("/data/{}", account.user)).exists() {
                return Err(error::ErrorConflict("User Conflict"))
            } else {true}
        },
        _ => {
            let pw_hash_file = format!("/data/{}/password", account.user);
            if Path::new(&pw_hash_file).exists() {
                let mut file = File::open(&pw_hash_file)?;
		let mut buffer = [0; 32];
		file.read(&mut buffer)?;
		let hash = sha2::Sha256::digest(&account.password.as_bytes());
		hash.as_slice() == buffer
            } else {false}
        }
    };
    if auth {
        Ok(Json(Token {token: encode(&Header::default(), &claim, "secret".as_ref()).unwrap()}))
    } else {
        Err(error::ErrorUnauthorized("Unauthorized"))
    }
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
            r.post().with(login);
            r.put().f(renew);
        })
}
