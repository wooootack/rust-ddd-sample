use crate::scenarios::*;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .configure(init_users_routes)
    .configure(init_status_routes);
}

pub fn init_users_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/users", web::get().to(find_all_users_scenario::execute));
  cfg.route(
    "/users/{user_id}",
    web::get().to(find_one_user_scenario::execute),
  );
  cfg.route("/users", web::post().to(create_user_scenario::execute));
  cfg.route(
    "/users/{user_id}",
    web::put().to(update_user_scenario::execute),
  );

  cfg.route(
    "/users/{user_id}/tasks",
    web::post().to(create_task_scenario::execute),
  );
}

pub fn init_status_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/status", web::get().to(health_check_scenario::execute));
}
