use super::user::User;

pub trait UsersRepository {
    fn save(self, user: &User);
}

pub struct MockUsersRepository {}

impl UsersRepository for MockUsersRepository {
    fn save(self, _user: &User) {}
}
