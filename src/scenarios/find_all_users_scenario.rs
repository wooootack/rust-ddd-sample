use crate::{
  modules::{
    common::{api_error::ApiError, pagination::PageInfo},
    user::presentation::find_all_users::{self, FindAllUsersRequest},
  },
  scenarios::{find_one_user_scenario::UserScenarioResponse, request_context::RequestContext},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FindAllUsersScenarioRequest {
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub is_limit_all: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FindAllUsersScenarioResponse {
  pub items: Vec<UserScenarioResponse>,
  pub page_info: PageInfo,
}

pub async fn execute(
  context: web::Data<RequestContext>,
  request: Option<web::Query<FindAllUsersScenarioRequest>>,
) -> Result<HttpResponse, ApiError> {
  let request = match request {
    Some(req) => FindAllUsersRequest {
      page: req.page,
      limit: req.limit,
      is_limit_all: req.is_limit_all,
    },
    None => FindAllUsersRequest {
      page: None,
      limit: None,
      is_limit_all: None,
    },
  };

  let response = web::block(move || {
    let conn = context.get_connection();

    find_all_users::execute(conn, request)
  })
  .await??;

  let users: Vec<UserScenarioResponse> = response
    .items
    .into_iter()
    .map(UserScenarioResponse::from)
    .collect();

  Ok(HttpResponse::Ok().json(FindAllUsersScenarioResponse {
    items: users,
    page_info: response.page_info,
  }))
}
