use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::Error as ActixWebError;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::error;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiError {
  pub status_code: u16,
  pub message: String,
}

impl ApiError {
  pub fn new(status_code: StatusCode, message: String) -> ApiError {
    ApiError {
      status_code: status_code.as_u16(),
      message,
    }
  }
}

impl fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

impl From<DieselError> for ApiError {
  fn from(error: DieselError) -> ApiError {
    match error {
      DieselError::DatabaseError(_, err) => {
        ApiError::new(StatusCode::CONFLICT, err.message().to_string())
      }
      DieselError::NotFound => ApiError::new(StatusCode::NOT_FOUND, "Record not found".to_string()),
      err => ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Diesel error: {err}"),
      ),
    }
  }
}

impl From<BlockingError> for ApiError {
  fn from(error: BlockingError) -> Self {
    // MEMO: error.to_string()で何が出るのか確認できていないけど、FW側で実装してくれているTraitなのでおかしな文字列は出ないはず
    ApiError::new(error.status_code(), error.to_string())
  }
}

impl From<ActixWebError> for ApiError {
  fn from(error: ActixWebError) -> Self {
    let res = error.as_error::<ApiError>();

    match res {
      Some(api_error) => ApiError {
        status_code: api_error.status_code,
        message: api_error.message.to_string(),
      },
      None => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
  }
}

impl From<Box<dyn Error + Send + Sync>> for ApiError {
  fn from(_: Box<dyn Error + Send + Sync>) -> Self {
    ApiError::new(
      StatusCode::INTERNAL_SERVER_ERROR,
      "想定外のエラーが発生しました".to_owned(),
    )
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    let status_code = match StatusCode::from_u16(self.status_code) {
      Ok(status_code) => status_code,
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let message = match status_code.as_u16() < 500 {
      true => self.message.clone(),
      false => {
        error!("{}", self.message);
        "Internal server error".to_string()
      }
    };

    HttpResponse::build(status_code).json(json!({ "message": message }))
  }
}
