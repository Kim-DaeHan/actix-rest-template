use actix_web::{
    web::{get, scope, Data},
    App, HttpResponse, HttpServer,
};
use diesel::RunQueryDsl;
use my_actix_app::*;

pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = establish_connection();
    let mut connection = pool.get().expect("Failed to get connection from pool");

    match diesel::sql_query("SELECT 1").execute(&mut connection) {
        Ok(_) => println!("Database connection successful!"),
        Err(err) => eprintln!("Error connecting to the database: {:?}", err),
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(scope("/api").configure(routes::configure))
            .route(
                "/",
                get().to(|| async { HttpResponse::Ok().body("Hello, Actix!") }),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
