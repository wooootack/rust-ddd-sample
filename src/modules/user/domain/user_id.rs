use serde::Serialize;
use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct UserId {
  pub value: String,
}

impl Default for UserId {
  fn default() -> Self {
    let value = Ulid::new().to_string();
    Self { value }
  }
}

impl TryFrom<String> for UserId {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let ulid = Ulid::from_string(&value);
    match ulid {
      Ok(value) => Ok(Self {
        value: value.to_string(),
      }),
      Err(err) => Err(format!("can't convert to AdminId. error: {err}")),
    }
  }
}

impl UserId {
  pub fn restore(value: String) -> Self {
    let value = Ulid::from_string(&value).unwrap().to_string();
    Self { value }
  }
}
