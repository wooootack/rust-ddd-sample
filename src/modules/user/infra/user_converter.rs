use crate::modules::{
  diesel::user::User,
  user::domain::{user::User as DomainUser, user_id::UserId, user_name::UserName},
};

impl From<User> for DomainUser {
  fn from(user: User) -> Self {
    DomainUser::restore(
      UserId::restore(user.id.clone()),
      UserName::restore(user.first_name.clone(), user.last_name.clone()),
      user.mail_address.clone(),
      user.age,
      user.created_at,
      user.updated_at,
    )
  }
}

impl From<DomainUser> for User {
  fn from(domain_user: DomainUser) -> Self {
    User {
      id: domain_user.id.value.clone(),
      first_name: domain_user.name.first_name.clone(),
      last_name: domain_user.name.last_name.clone(),
      mail_address: domain_user.mail_address.clone(),
      age: domain_user.age,
      created_at: domain_user.created_at,
      updated_at: domain_user.updated_at,
    }
  }
}
