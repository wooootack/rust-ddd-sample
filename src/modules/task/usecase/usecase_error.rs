use crate::modules::{
  common::api_error::ApiError,
  task::domain::domain_error::{DomainError, DomainErrorType},
};
use actix_web::http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct UsecaseError {
  pub error_type: UsecaseErrorType,
  pub message: String,
}

impl fmt::Display for UsecaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

#[derive(Debug)]
pub enum UsecaseErrorType {
  Invalid,
  NotFound,
  NotAuthorized,
  Other,
}

impl UsecaseError {
  pub fn new(error_type: UsecaseErrorType, message: String) -> UsecaseError {
    UsecaseError {
      error_type,
      message,
    }
  }
  pub fn invalid(message: String) -> UsecaseError {
    UsecaseError {
      error_type: UsecaseErrorType::Invalid,
      message,
    }
  }
  pub fn not_found(message: String) -> UsecaseError {
    UsecaseError {
      error_type: UsecaseErrorType::NotFound,
      message,
    }
  }
  pub fn not_authorized(message: String) -> UsecaseError {
    UsecaseError {
      error_type: UsecaseErrorType::NotFound,
      message,
    }
  }
  pub fn other(message: String) -> UsecaseError {
    UsecaseError {
      error_type: UsecaseErrorType::Other,
      message,
    }
  }
}

impl From<DomainError> for UsecaseError {
  fn from(error: DomainError) -> Self {
    let error_type = match error.error_type {
      DomainErrorType::Invalid => UsecaseErrorType::Invalid,
      DomainErrorType::NotFound => UsecaseErrorType::NotFound,
      _ => UsecaseErrorType::Other,
    };
    UsecaseError::new(error_type, error.message)
  }
}

// MEMO: ApiErrorはmoduleを超えて共通のため、このmodule内で独自に変換処理を書く
impl From<UsecaseError> for ApiError {
  fn from(error: UsecaseError) -> Self {
    let status_code = match error.error_type {
      UsecaseErrorType::Invalid => StatusCode::BAD_REQUEST,
      UsecaseErrorType::NotFound => StatusCode::NOT_FOUND,
      UsecaseErrorType::NotAuthorized => StatusCode::FORBIDDEN,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    ApiError::new(status_code, error.to_string())
  }
}
