use actix_web::{web, App, HttpServer};
use my_actix_app::*;

// mod handlers;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        let connection = establish_connection();
        App::new().app_data(web::Data::new(connection))
        // .configure(routes::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
