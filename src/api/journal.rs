use actix_web::{
    App,
    HttpRequest,
    HttpResponse
};

fn list_journal(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    print!("{:?}", req.match_info());
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
    print!("{:?}", req);
    print!("{:?}", req.match_info());
    HttpResponse::Ok().into()
}

pub fn handlers(base_url: &str) -> App {
    App::new()
        .prefix(base_url)
        .resource("/public/{journal_name}", |r| {
            r.get().f(public_journal);})
        .resource("/{author}/{journal_name}", |r| {
            r.get().f(journal_detail);})
        .resource("/", |r| {
            r.get().f(list_journal);
            r.post().f(create_journal);
        })
}
