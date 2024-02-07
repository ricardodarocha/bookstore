mod entidade;
mod prelude;
mod error;

use prelude::*;
use axum::{Extension, Router, routing::{get, put, post}, extract::{Path, Json}};//State,
use eyre::WrapErr;
use sqlx::MySqlPool;
use entidade::{SelectSocioDep, MergeSocioDep, InsertSocioDep};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")
        .wrap_err("DATABASE_URL must be set")?;

    let db = MySqlPool::connect(&database_url).await?;

    let app = Router::new()
    .route("/sociodep", post(sociodep_post)) //Insert 
    .route("/sociodep", get(sociodep_get_all)) //Select all
    .route("/sociodep/:id", get(sociodep_get)) //Select one
    .route("/sociodep/:id", put(sociodep_put)) //Update
    .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3048").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn sociodep_put(db: Extension<MySqlPool>, Path(id): Path<String>, Json(payload): Json<MergeSocioDep>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let mut socio = dbg!(SelectSocioDep::read(&db, id).await?);
    let payload = dbg!(payload);

    socio.patch(payload);

    dbg!(&socio);

    socio.write(&db).await?;

    Ok(Json(socio))
}

async fn sociodep_post(db: Extension<MySqlPool>, Json(payload): Json<InsertSocioDep>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let payload = dbg!(payload);
    let socio = dbg!(InsertSocioDep::create(&payload, &db).await?);

    dbg!(&socio);

    socio.write(&db).await?;

    Ok(Json(socio))
}

async fn sociodep_get(db: Extension<MySqlPool>, Path(id): Path<String>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let socio = dbg!(SelectSocioDep::read(&db, id).await?);
    dbg!(&socio);
    Ok(Json(socio))
}

async fn sociodep_get_all(db: Extension<MySqlPool>) -> Result<Json<Vec<SelectSocioDep>>> where  Result<Json<Vec<SelectSocioDep>>>: axum::response::IntoResponse {
    let socio = dbg!(SelectSocioDep::read_all(&db).await?);
    dbg!(&socio);
    Ok(Json(socio))
}