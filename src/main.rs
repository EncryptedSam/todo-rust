mod db;
mod schema;
mod utils;
mod v1;

use actix_web::{web, App, HttpServer};
use db::connection::connection;
// use utils::decode_jwt_token;
use v1::routes::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}