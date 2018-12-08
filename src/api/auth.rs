use actix_web::{
    App,
    HttpRequest,
    HttpResponse
};

fn login(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    print!("{:?}", req.match_info());
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
