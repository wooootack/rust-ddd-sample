use super::find_all_users::UsecaseError;
use crate::modules::user::domain::{user::User, users_repository::UsersRepository};

pub struct FindUserByIdRequest {
    pub id: String,
}

pub struct FindUserByIdResponse {
    pub user: Option<User>,
}

pub struct FindUserByIdUsecase<Repository: UsersRepository> {
    users_repository: Repository,
}

impl<T: UsersRepository> FindUserByIdUsecase<T> {
    pub fn new(repository: T) -> Self {
        FindUserByIdUsecase {
            users_repository: repository,
        }
    }

    pub fn execute(
        self,
        request: FindUserByIdRequest,
    ) -> Result<FindUserByIdResponse, UsecaseError> {
        let user = self.users_repository.find_by_id(&request.id)?;

        Ok(FindUserByIdResponse { user })
    }
}
