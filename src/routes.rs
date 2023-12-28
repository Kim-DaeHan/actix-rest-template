use crate::handlers::post_handler;
use actix_web::{http::header::ContentType, web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/posts")
            .route(web::get().to(post_handler::get_posts))
            .route(web::post().to(post_handler::create_posts)),
    );
    cfg.service(web::resource("/posts/{id}").route(web::get().to(post_handler::get_posts_by_id)));
    cfg.service(web::resource("/").route(web::get().to(my_handler)));
}

async fn my_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("Hello, Actix!222222가나다라")
}
