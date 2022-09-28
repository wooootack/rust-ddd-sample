use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub struct DomainError {
  pub error_type: DomainErrorType,
  pub message: String,
}

impl fmt::Display for DomainError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

#[derive(Debug)]
pub enum DomainErrorType {
  Invalid,
  NotFound,
  Other,
}

impl DomainError {
  pub fn invalid(message: String) -> DomainError {
    DomainError {
      error_type: DomainErrorType::Invalid,
      message,
    }
  }
  pub fn not_found(message: String) -> DomainError {
    DomainError {
      error_type: DomainErrorType::NotFound,
      message,
    }
  }
  pub fn other(message: String) -> DomainError {
    DomainError {
      error_type: DomainErrorType::Other,
      message,
    }
  }
}

// MEMO: 本来はinfra層でこの定義をするのが良さそう
impl From<DieselError> for DomainError {
  fn from(error: DieselError) -> DomainError {
    match error {
      DieselError::NotFound => DomainError::not_found("Not found".to_string()),
      err => DomainError::other(format!("Diesel error: {err}")),
    }
  }
}
