use actix_web::{App, HttpServer};
use rust_ddd_sample::modules::user::presentation::users_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(users_controller::get_all_users)
            .service(users_controller::create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
