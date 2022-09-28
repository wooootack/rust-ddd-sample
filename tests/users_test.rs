pub mod common;

#[cfg(feature = "integration_test")]
mod tests {
  use crate::common::{byte::AsStr, error_response::ErrorResponse};
  use actix_web::{body, http::StatusCode, test, web, App};
  use chrono::{TimeZone, Utc};
  use rust_ddd_sample::{
    modules::common::pagination::PageInfo,
    routes,
    scenarios::{
      find_all_users_scenario::FindAllUsersScenarioResponse,
      find_one_user_scenario::UserScenarioResponse, request_context::RequestContext,
    },
  };

  #[actix_web::test]
  async fn success_find_all_users() {
    let app = test::init_service(
      App::new()
        .app_data(web::Data::new(RequestContext::default()))
        .configure(routes::init_users_routes),
    )
    .await;

    let res = test::TestRequest::get()
      .uri("/users?is_limit_all=true")
      .send_request(&app)
      .await;

    assert_eq!(res.status(), StatusCode::OK);

    let mock_date = Utc
      .datetime_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S")
      .unwrap();

    let actual: FindAllUsersScenarioResponse = test::read_body_json(res).await;
    let expected = FindAllUsersScenarioResponse {
      items: vec![UserScenarioResponse {
        id: "01GSJ8XX68PXMNKGFWTM4G06MQ".to_owned(),
        first_name: "太郎".to_owned(),
        last_name: "山田".to_owned(),
        mail_address: "taro_yamada@example.com".to_owned(),
        age: 25,
        created_at: mock_date,
        updated_at: mock_date,
      }],
      page_info: PageInfo {
        has_next_page: false,
        has_prev_page: false,
        page: 1,
        limit: 100,
        total_count: 1,
      },
    };

    assert_eq!(actual.page_info, expected.page_info);
    assert_eq!(actual.items.len(), expected.items.len());

    // itemsのアサーションは省略
  }

  #[actix_web::test]
  async fn success_find_one_user_not_found() {
    let app = test::init_service(
      App::new()
        .app_data(web::Data::new(RequestContext::default()))
        .configure(routes::init_users_routes),
    )
    .await;

    let res = test::TestRequest::get()
      .uri("/users/01GSJ8XX68PXMNKGFWT9999999")
      .send_request(&app)
      .await;

    assert_eq!(res.status(), StatusCode::OK);

    let body = body::to_bytes(res.into_body()).await.unwrap();
    assert_eq!(body.as_str(), "User not found".to_string());
  }

  #[actix_web::test]
  async fn failure_403_create_user() {
    let app = test::init_service(
      App::new()
        .app_data(web::Data::new(RequestContext::default()))
        .configure(routes::init_users_routes),
    )
    .await;

    let res = test::TestRequest::post()
      .uri("/users")
      .send_request(&app)
      .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
    let res: ErrorResponse = test::read_body_json(res).await;
    let expected = ErrorResponse {
      message: "forbidden".to_owned(),
    };
    assert_eq!(res, expected);
  }
}
