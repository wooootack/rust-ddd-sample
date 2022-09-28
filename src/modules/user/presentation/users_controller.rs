use actix_web::{delete, get, http::Error, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]

pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub age: i16,
}

#[derive(Deserialize)]

pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub age: i16,
}

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
pub async fn create_user(
    pool: web::Data<DbPool>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    let param = CreateUserParameter {
        first_name: request.first_name.to_string(),
        last_name: request.last_name.to_string(),
        age: request.age,
    };

    let response = web::block(move || {
        let mut conn = pool.get()?;
        let usecase = CreateUserUseCase::new(PostgresUsersRepository::new(&mut conn));
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

#[post("/users/{user_id}")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
    request: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("updated user."))
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("deleted user."))
}
