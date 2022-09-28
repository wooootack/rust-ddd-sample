use crate::modules::user::domain::{user::User, users_repository::UsersRepository};

pub type UsecaseError = Box<dyn std::error::Error + Send + Sync>;

pub struct FindAllUsersResponse {
    // とりあえずドメインオブジェクトをそのまま返す
    pub users: Vec<User>,
}

pub struct FindAllUsersUsecase<Repository: UsersRepository> {
    user_repository: Repository,
}

impl<T: UsersRepository> FindAllUsersUsecase<T> {
    pub fn new(repository: T) -> Self {
        FindAllUsersUsecase {
            user_repository: repository,
        }
    }

    pub fn execute(self) -> Result<FindAllUsersResponse, UsecaseError> {
        let users = self.user_repository.find_all()?;

        Ok(FindAllUsersResponse { users })
    }
}

// #[cfg(test)]

// mod tests {

//     use crate::modules::user::domain::users_repository::MockUsersRepository;

//     use super::*;

//     #[test]
//     fn success_execute() {
//         let create_user_usecase = CreateUserUseCase::new(MockUsersRepository {});

//         create_user_usecase.execute(CreateUserParameter {
//             first_name: "John".to_string(),
//             last_name: "Doe".to_string(),
//             age: 30,
//         });
//     }
// }
