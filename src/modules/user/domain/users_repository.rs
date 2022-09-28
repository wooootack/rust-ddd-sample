use crate::modules::user::domain::{user_id::UserId, user_name::UserName};

use super::user::User;

pub trait UsersRepository {
    fn find_all(self) -> Vec<User>;

    fn find_by_id(self, id: &str) -> Option<User>;

    fn register(self, user: &User);

    fn update(self, user: &User);

    fn delete(self, user: &User);
}

pub struct MockUsersRepository {}

impl UsersRepository for MockUsersRepository {
    fn find_all(self) -> Vec<User> {
        let id = UserId::default();
        let name = UserName::new("John".to_string(), "Doe".to_string());
        let age = 30;

        let results = vec![User::new(id, name, age)];

        results
    }

    fn find_by_id(self, _id: &str) -> Option<User> {
        let id = UserId::default();
        let name = UserName::new("John".to_string(), "Doe".to_string());
        let age = 30;

        Some(User::new(id, name, age))
    }

    fn register(self, _user: &User) {}

    fn update(self, _user: &User) {}

    fn delete(self, _user: &User) {}
}
