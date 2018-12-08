use std::fs;

use actix_web::{
    http,
    App,
    HttpRequest,
    HttpResponse
};

use serde_json;

fn list_journal(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    HttpResponse::Ok().into()
}

fn create_journal(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    print!("{:?}", req.match_info());
    HttpResponse::Ok().into()
}

fn journal_detail(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    print!("{:?}", req.match_info());
    HttpResponse::Ok().into()
}

fn public_journal(req: &HttpRequest) -> HttpResponse {
    let journals = fs::read_dir("/data/public").unwrap().filter_map(|entry| {
        entry.ok().and_then(|e|
            e.path().file_name()
                .and_then(|n|
                    n.to_str() .map(|s| String::from(s))
                )
        )
    }).collect::<Vec<String>>();
    HttpResponse::build(http::StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&journals).unwrap_or("[]".to_string()))
}

pub fn handlers(base_url: &str) -> App {
    App::new()
        .prefix(base_url)
        .resource("/public", |r| {
            r.get().f(public_journal);})
        .resource("/{author}/{journal_name}", |r| {
            r.get().f(journal_detail);})
        .resource("/?", |r| {
            r.get().f(list_journal);
            r.post().f(create_journal);
        })
}
