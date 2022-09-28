use crate::{
  modules::{
    common::pagination::{PageInfo, Paging},
    diesel::user::User,
    user::domain::{
      domain_error::DomainError,
      user::User as DomainUser,
      user_id::UserId,
      users_repository::{FindUsersResult, UsersRepository},
    },
  },
  scenarios::request_context::DBConn,
  schema::users::dsl::*,
};
use diesel::prelude::*;
use std::{cell::RefCell, ops::DerefMut, rc::Rc};

pub struct PostgresUsersRepository {
  conn: Rc<RefCell<DBConn>>,
}

impl PostgresUsersRepository {
  pub fn new(conn: Rc<RefCell<DBConn>>) -> Self {
    Self { conn }
  }

  fn count_all(&mut self) -> Result<i64, DomainError> {
    let count = users
      .count()
      .get_result::<i64>(self.conn.borrow_mut().deref_mut())?;
    Ok(count)
  }

  fn has_next(&mut self, paging: &Paging) -> Result<bool, DomainError> {
    match users
      .order(id.desc())
      .offset(paging.next_offset())
      .first::<User>(self.conn.borrow_mut().deref_mut())
    {
      Ok(_) => Ok(true),
      Err(diesel::result::Error::NotFound) => Ok(false),
      Err(e) => Err(e)?,
    }
  }

  fn has_prev(&mut self, paging: &Paging) -> Result<bool, DomainError> {
    if paging.prev_offset() < 0 {
      return Ok(false);
    }
    match users
      .order(id.desc())
      .offset(paging.prev_offset())
      .first::<User>(self.conn.borrow_mut().deref_mut())
    {
      Ok(_) => Ok(true),
      Err(diesel::result::Error::NotFound) => Ok(false),
      Err(e) => Err(e)?,
    }
  }
}

impl UsersRepository for PostgresUsersRepository {
  fn find_all(&mut self) -> Result<FindUsersResult, DomainError> {
    let results = users.load::<User>(self.conn.borrow_mut().deref_mut())?;

    let domain_users: Vec<DomainUser> = results.into_iter().map(DomainUser::from).collect();

    let items_count = domain_users.len() as i64;

    Ok(FindUsersResult {
      items: domain_users,
      page_info: PageInfo::from_all(items_count),
    })
  }

  fn find(&mut self, paging: &Paging) -> Result<FindUsersResult, DomainError> {
    let results = users
      .order_by(id.desc())
      .offset(paging.offset())
      .limit(paging.limit())
      .load::<User>(self.conn.borrow_mut().deref_mut())?;

    let domain_users: Vec<DomainUser> = results.into_iter().map(DomainUser::from).collect();

    Ok(FindUsersResult {
      items: domain_users,
      page_info: PageInfo::new(
        self.has_next(paging)?,
        self.has_prev(paging)?,
        paging.page(),
        paging.limit(),
        self.count_all()?,
      ),
    })
  }

  fn find_by_id(&mut self, user_id: &UserId) -> Result<Option<DomainUser>, DomainError> {
    match users
      .filter(id.eq(user_id.value.clone()))
      .first::<User>(self.conn.borrow_mut().deref_mut())
    {
      Ok(a) => Ok(Some(DomainUser::from(a))),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e)?,
    }
  }

  fn find_by_email(&mut self, email: &str) -> Result<Option<DomainUser>, DomainError> {
    match users
      .filter(mail_address.eq(email))
      .first::<User>(self.conn.borrow_mut().deref_mut())
    {
      Ok(a) => Ok(Some(DomainUser::from(a))),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e)?,
    }
  }

  fn register(&mut self, user: &DomainUser) -> Result<(), DomainError> {
    diesel::insert_into(users)
      .values(User::from(user.clone()))
      .execute(self.conn.borrow_mut().deref_mut())?;

    Ok(())
  }

  fn update(&mut self, user: &DomainUser) -> Result<(), DomainError> {
    diesel::update(users.find(user.id.value.clone()))
      .set((
        first_name.eq(user.name.first_name.clone()),
        last_name.eq(user.name.last_name.clone()),
        age.eq(user.age),
      ))
      .execute(self.conn.borrow_mut().deref_mut())?;

    Ok(())
  }

  fn delete(&mut self, user: &DomainUser) -> Result<i16, DomainError> {
    let num_deleted = diesel::delete(users.find(user.id.value.clone()))
      .execute(self.conn.borrow_mut().deref_mut())?;

    Ok(num_deleted as i16)
  }
}
