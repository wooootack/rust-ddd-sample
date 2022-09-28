use actix_web::web::Bytes;

pub trait AsStr {
  fn as_str(&self) -> &str;
}
impl AsStr for Bytes {
  fn as_str(&self) -> &str {
    std::str::from_utf8(self).unwrap()
  }
}
