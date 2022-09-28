use actix_web::{middleware::Logger, web, App, HttpServer};
use rust_ddd_sample::{routes, scenarios::request_context::RequestContext};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();
  env_logger::init();

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(web::Data::new(RequestContext::default()))
      .configure(routes::init_routes)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
