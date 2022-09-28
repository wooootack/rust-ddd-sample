#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UserName {
  pub first_name: String,
  pub last_name: String,
  pub full_name: String,
}

impl UserName {
  pub fn new(first_name: String, last_name: String) -> Self {
    if first_name.is_empty() || last_name.is_empty() {
      panic!("first_name and last_name must not be empty");
    }

    let full_name = format!("{} {}", first_name, last_name);
    Self {
      first_name,
      last_name,
      full_name,
    }
  }

  pub fn restore(first_name: String, last_name: String) -> Self {
    let full_name = format!("{} {}", first_name, last_name);
    Self {
      first_name,
      last_name,
      full_name,
    }
  }
}
