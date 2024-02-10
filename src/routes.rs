use crate::auth::{authorize, AuthPayload, AuthenticatedUser};
use crate::prelude::*;
use crate::entidade::{SelectSocioDep, MergeSocioDep, InsertSocioDep};
use axum::extract::{Path, Json};
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::MySqlPool;

use axum::debug_handler;

#[debug_handler]
pub async fn login(payload: Json<AuthPayload>) -> impl IntoResponse {
    authorize(payload).await.into_response()
}


#[debug_handler]
pub async fn sociodep_put(db: Extension<MySqlPool>, Path(id): Path<i32>, user: AuthenticatedUser, Json(payload): Json<MergeSocioDep>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let mut socio = dbg!(SelectSocioDep::read(&db, id).await?);
    let payload = dbg!(payload);

    socio.patch(payload);

    dbg!(&socio);

    socio.update(&db).await?;

    Ok(Json(socio))
}

#[debug_handler]
pub async fn sociodep_post(db: Extension<MySqlPool>, user: AuthenticatedUser, Json(payload): Json<InsertSocioDep>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let payload = dbg!(payload);
    let socio = dbg!(InsertSocioDep::create(&payload, &db).await?);

    dbg!(&socio);

    Ok(Json(socio))
}

pub async fn sociodep_get(db: Extension<MySqlPool>, Path(id): Path<i32>, user: AuthenticatedUser) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let socio = dbg!(SelectSocioDep::read(&db, id).await?);
    dbg!(&socio);
    Ok(Json(socio))
}

pub async fn sociodep_get_all(db: Extension<MySqlPool>, user: AuthenticatedUser) -> Result<Json<Vec<SelectSocioDep>>> where  Result<Json<Vec<SelectSocioDep>>>: axum::response::IntoResponse {
    let socio = dbg!(SelectSocioDep::read_all(&db).await?);
    dbg!(&socio);
    Ok(Json(socio))
}