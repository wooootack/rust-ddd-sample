use crate::{
  modules::{
    common::{
      api_error::ApiError,
      pagination::{PageInfo, Paging},
    },
    user::{
      infra::postgres_users_repository::PostgresUsersRepository,
      presentation::find_one_user::UserResponse,
      usecase::{
        find_all_users_pagination_usecase::FindAllUsersPaginationUsecase,
        find_all_users_usecase::FindAllUsersUsecase,
      },
    },
  },
  scenarios::request_context::DBConn,
};
use std::{cell::RefCell, rc::Rc};

pub struct FindAllUsersRequest {
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub is_limit_all: Option<bool>,
}

pub struct FindAllUsersResponse {
  pub items: Vec<UserResponse>,
  pub page_info: PageInfo,
}

pub fn execute(
  conn: DBConn,
  request: FindAllUsersRequest,
) -> Result<FindAllUsersResponse, ApiError> {
  let conn = Rc::new(RefCell::new(conn));
  let tasks_repository = RefCell::new(PostgresUsersRepository::new(conn));

  let result = match request.is_limit_all {
    Some(is_limit_all) if is_limit_all => {
      let usecase = FindAllUsersPaginationUsecase::new(tasks_repository);
      let paging = Paging::new(request.page, request.limit, request.is_limit_all);
      usecase.execute(&paging)?
    }
    _ => {
      let usecase = FindAllUsersUsecase::new(tasks_repository);
      usecase.execute()?
    }
  };

  let users = result.items.into_iter().map(UserResponse::from).collect();

  Ok(FindAllUsersResponse {
    items: users,
    page_info: result.page_info,
  })
}
