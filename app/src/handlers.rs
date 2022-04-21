use crate::{db, errors::MyError, models::User};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let current_users = db::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(current_users))
}
