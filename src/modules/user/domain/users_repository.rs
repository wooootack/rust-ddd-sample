use crate::modules::user::domain::{user_id::UserId, user_name::UserName};

use super::user::User;

pub type RepositoryError = Box<dyn std::error::Error + Send + Sync>;

pub trait UsersRepository {
    fn find_all(self) -> Result<Vec<User>, RepositoryError>;

    fn find_by_id(self, id: &str) -> Result<Option<User>, RepositoryError>;

    fn register(self, user: &User);

    fn update(self, user: &User);

    fn delete(self, user: &User);
}

pub struct MockUsersRepository {}

impl UsersRepository for MockUsersRepository {
    fn find_all(self) -> Result<Vec<User>, RepositoryError> {
        let id = UserId::default();
        let name = UserName::new("John".to_string(), "Doe".to_string());
        let age = 30;

        let results = vec![User::new(id, name, age)];

        Ok(results)
    }

    fn find_by_id(self, _id: &str) -> Result<Option<User>, RepositoryError> {
        let id = UserId::default();
        let name = UserName::new("John".to_string(), "Doe".to_string());
        let age = 30;

        Ok(Some(User::new(id, name, age)))
    }

    fn register(self, _user: &User) {}

    fn update(self, _user: &User) {}

    fn delete(self, _user: &User) {}
}
