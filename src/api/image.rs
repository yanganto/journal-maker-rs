use actix_web::{
    App,
    HttpRequest,
    HttpResponse
};

fn save_image(req: &HttpRequest) -> HttpResponse {
    print!("{:?}", req);
    print!("{:?}", req.match_info());
    HttpResponse::Ok().into()
}

pub fn handlers(base_url: &str) -> App {
    App::new()
        .prefix(base_url)
        .resource("/{image_name}", |r| {
            r.post().f(save_image);})
}
