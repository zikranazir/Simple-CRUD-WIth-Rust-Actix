use actix_web::{web, App, HttpServer};
mod models;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/items", web::post().to(handlers::create_item))
            .route("/get_all_items", web::get().to(handlers::get_all_items))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
