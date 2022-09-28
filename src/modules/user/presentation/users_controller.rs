use actix_web::{get, post, HttpResponse, Responder};

use crate::modules::user::{
    domain::users_repository::MockUsersRepository,
    usecase::create_user::{CreateUserParameter, CreateUserUseCase},
};

#[get("/users")]
pub async fn get_all_users() -> impl Responder {
    HttpResponse::Ok().body("Get All Users.")
}

#[post("/users")]
pub async fn create_user() -> impl Responder {
    let usecase = CreateUserUseCase::new(MockUsersRepository {});

    let param = CreateUserParameter {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };

    let res = usecase.execute(param);

    HttpResponse::Ok().body(format!(
        "user_id: {}, user_name: {}, age:  {}",
        res.user_id, res.user_name, res.age
    ))
}
