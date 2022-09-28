use crate::modules::user::domain::{
    user::User, user_id::UserId, user_name::UserName, users_repository::UsersRepository,
};

pub struct CreateUserParameter {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

pub struct CreateUserResponse {
    pub user_id: String,
    pub user_name: String,
    pub age: u8,
}

pub struct CreateUserUseCase<Repository: UsersRepository> {
    user_repository: Repository,
}

impl<T: UsersRepository> CreateUserUseCase<T> {
    pub fn new(repository: T) -> Self {
        CreateUserUseCase {
            user_repository: repository,
        }
    }

    pub fn execute(self, parameter: CreateUserParameter) -> CreateUserResponse {
        let user_id = UserId::default();
        let user_name = UserName::new(parameter.first_name, parameter.last_name);
        let user = User::new(user_id, user_name, parameter.age);
        self.user_repository.save(&user);

        CreateUserResponse {
            user_id: user.id.value.to_string(),
            user_name: user.name.full_name,
            age: user.age,
        }
    }
}

#[cfg(test)]

mod tests {

    use crate::modules::user::domain::users_repository::MockUsersRepository;

    use super::*;

    #[test]
    fn success_execute() {
        let create_user_usecase = CreateUserUseCase::new(MockUsersRepository {});

        create_user_usecase.execute(CreateUserParameter {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 30,
        });
    }
}
