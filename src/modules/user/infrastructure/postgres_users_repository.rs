use diesel::PgConnection;

use crate::modules::user::domain::user_id::UserId;
use crate::modules::user::domain::user_name::UserName;
use crate::modules::user::domain::{user::User as DomainUser, users_repository::UsersRepository};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i16,
}

pub struct PostgresUsersRepository {
    connection: PgConnection,
}

impl PostgresUsersRepository {
    pub fn new(connection: PgConnection) -> Self {
        PostgresUsersRepository { connection }
    }
}

impl UsersRepository for PostgresUsersRepository {
    fn find_all(self) -> Vec<DomainUser> {
        let mut conn = self.connection;

        let results = users.load::<User>(&mut conn).expect("Error loading users");

        results
            .iter()
            .map(|user| {
                let user_id = UserId::default();
                let user_name =
                    UserName::new(user.first_name.to_string(), user.last_name.to_string());

                DomainUser::new(user_id, user_name, user.age)
            })
            .collect()
    }

    fn find_by_id(self, user_id: &str) -> Option<DomainUser> {
        let mut conn = self.connection;

        let results = users
            .filter(id.eq(user_id))
            .load::<User>(&mut conn)
            .expect("Error loading users");

        results
            .iter()
            .map(|user| {
                let user_id = UserId::default();
                let user_name =
                    UserName::new(user.first_name.to_string(), user.last_name.to_string());

                DomainUser::new(user_id, user_name, user.age)
            })
            .collect::<Vec<DomainUser>>()
            .first()
            .cloned()
    }

    fn register(self, user: &DomainUser) {
        todo!()
    }

    fn update(self, user: &DomainUser) {
        todo!()
    }

    fn delete(self, user: &DomainUser) {
        todo!()
    }
}
