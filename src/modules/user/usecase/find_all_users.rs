use crate::modules::user::domain::{user::User, users_repository::UsersRepository};

pub type UsecaseError = Box<dyn std::error::Error + Send + Sync>;

pub struct FindAllUsersResponse {
    // とりあえずドメインオブジェクトをそのまま返す
    pub users: Vec<User>,
}

pub struct FindAllUsersUsecase<Repository: UsersRepository> {
    users_repository: Repository,
}

impl<T: UsersRepository> FindAllUsersUsecase<T> {
    pub fn new(repository: T) -> Self {
        FindAllUsersUsecase {
            users_repository: repository,
        }
    }

    pub fn execute(self) -> Result<FindAllUsersResponse, UsecaseError> {
        let users = self.users_repository.find_all()?;

        Ok(FindAllUsersResponse { users })
    }
}
