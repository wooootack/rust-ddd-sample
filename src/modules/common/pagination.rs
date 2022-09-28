use serde::{Deserialize, Serialize};

pub struct Paging {
  pub page: i64,
  pub limit: i64,
  pub is_limit_all: bool,
}

const DEFAULT_PAGE: i64 = 1;
const MIN_PAGE: i64 = 1;
const DEFAULT_LIMIT: i64 = 100;
const MIN_LIMIT: i64 = 1;
const DEFAULT_LIMIT_ALL: bool = false;

impl Paging {
  pub fn new(page: Option<i64>, limit: Option<i64>, is_limit_all: Option<bool>) -> Self {
    Self {
      page: match page {
        None => DEFAULT_PAGE,
        Some(n) if n < MIN_PAGE => DEFAULT_PAGE,
        Some(n) => n,
      },
      limit: match limit {
        None => DEFAULT_LIMIT,
        Some(n) if n < MIN_LIMIT => DEFAULT_LIMIT,
        Some(n) => n,
      },
      is_limit_all: match is_limit_all {
        Some(b) => b,
        None => DEFAULT_LIMIT_ALL,
      },
    }
  }

  pub fn next_offset(&self) -> i64 {
    self.limit * (self.page)
  }

  pub fn prev_offset(&self) -> i64 {
    self.limit * (self.page - 2)
  }

  pub fn offset(&self) -> i64 {
    self.limit * (self.page - 1)
  }

  pub fn limit(&self) -> i64 {
    self.limit
  }

  pub fn page(&self) -> i64 {
    self.page
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PageInfo {
  pub has_next_page: bool,
  pub has_prev_page: bool,
  pub page: i64,
  pub limit: i64,
  pub total_count: i64,
}

impl PageInfo {
  pub fn new(
    has_next_page: bool,
    has_prev_page: bool,
    page: i64,
    limit: i64,
    total_count: i64,
  ) -> Self {
    Self {
      has_next_page,
      has_prev_page,
      page,
      limit,
      total_count,
    }
  }

  pub fn from_all(items_count: i64) -> Self {
    Self {
      has_next_page: false,
      has_prev_page: false,
      page: 1,
      limit: 0,
      total_count: items_count,
    }
  }
}
