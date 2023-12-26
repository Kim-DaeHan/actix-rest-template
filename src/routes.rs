use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users").route(web::get().to(my_handler)),
        // .route(web::get().to(user_handler::get_users))
        // .route(web::post().to(user_handler::create_user)),
    );

    cfg.service(web::resource("/").route(web::get().to(my_handler)));
}

async fn my_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}
