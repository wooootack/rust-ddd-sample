use actix_web::{
    error::ErrorInternalServerError, get, http::Error, post, web, HttpResponse, Responder,
};

use crate::{
    modules::user::{
        domain::users_repository::MockUsersRepository,
        infrastructure::postgres_users_repository::PostgresUsersRepository,
        usecase::{
            create_user::{CreateUserParameter, CreateUserUseCase},
            find_all_users::FindAllUsersUsecase,
            find_user_by_id::{FindUserByIdRequest, FindUserByIdUsecase},
        },
    },
    DbPool,
};

#[get("/users")]
pub async fn get_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let response = web::block(move || {
        let mut conn = pool.get()?;
        let usecase = FindAllUsersUsecase::new(PostgresUsersRepository::new(&mut conn));
        usecase.execute()
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError);

    match response {
        Ok(response) => match response {
            Ok(response) => Ok(HttpResponse::Ok().json(response.users)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let param = FindUserByIdRequest {
        id: user_id.to_string(),
    };

    let response = web::block(move || {
        let mut conn = pool.get()?;
        let usecase = FindUserByIdUsecase::new(PostgresUsersRepository::new(&mut conn));
        usecase.execute(param)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError);

    match response {
        Ok(response) => match response {
            Ok(response) => Ok(HttpResponse::Ok().json(response.user)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
        },
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
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
