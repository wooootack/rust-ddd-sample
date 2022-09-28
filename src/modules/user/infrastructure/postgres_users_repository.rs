use diesel::PgConnection;

use crate::modules::user::domain::user_id::UserId;
use crate::modules::user::domain::user_name::UserName;
use crate::modules::user::domain::users_repository::RepositoryError;
use crate::modules::user::domain::{user::User as DomainUser, users_repository::UsersRepository};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i16,
}

pub struct PostgresUsersRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> PostgresUsersRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        PostgresUsersRepository { connection }
    }
}

impl UsersRepository for PostgresUsersRepository<'_> {
    fn find_all(self) -> Result<Vec<DomainUser>, RepositoryError> {
        let conn = self.connection;

        let results = users.load::<User>(conn).expect("Error loading users");

        let domain_users: Vec<DomainUser> = results
            .iter()
            .map(|user| {
                let user_id = UserId::restore(&user.id.to_string());
                let user_name =
                    UserName::new(user.first_name.to_string(), user.last_name.to_string());

                DomainUser::new(user_id, user_name, user.age)
            })
            .collect();

        Ok(domain_users)
    }

    fn find_by_id(self, user_id: &str) -> Result<Option<DomainUser>, RepositoryError> {
        let conn = self.connection;

        let results = users
            .filter(id.eq(user_id))
            .load::<User>(conn)
            .expect("Error loading users");

        let domain_user = results
            .iter()
            .map(|user| {
                let user_id = UserId::default();
                let user_name =
                    UserName::new(user.first_name.to_string(), user.last_name.to_string());

                DomainUser::new(user_id, user_name, user.age)
            })
            .collect::<Vec<DomainUser>>()
            .first()
            .cloned();

        Ok(domain_user)
    }

    fn register(self, user: &DomainUser) -> Result<DomainUser, RepositoryError> {
        let conn = self.connection;

        let new_user = User {
            id: user.id.value.clone(),
            first_name: user.name.first_name.clone(),
            last_name: user.name.last_name.clone(),
            age: user.age,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");

        Ok(user.clone())
    }

    fn update(self, user: &DomainUser) -> Result<DomainUser, RepositoryError> {
        let conn = self.connection;

        diesel::update(users.find(user.id.value.clone()))
            .set((
                first_name.eq(user.name.first_name.clone()),
                last_name.eq(user.name.last_name.clone()),
                age.eq(user.age),
            ))
            .execute(conn)
            .expect("Error updating user");

        Ok(user.clone())
    }

    fn delete(self, user: &DomainUser) -> Result<i16, RepositoryError> {
        let conn = self.connection;

        let num_deleted = diesel::delete(users.find(user.id.value.clone()))
            .execute(conn)
            .expect("Error deleting user");

        Ok(num_deleted as i16)
    }
}
