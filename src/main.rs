use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use rust_ddd_sample::modules::user::presentation::users_controller;
use std::env;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(users_controller::get_all_users)
            .service(users_controller::create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
