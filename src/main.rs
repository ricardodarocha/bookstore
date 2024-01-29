#![allow(dead_code, unused)]

mod patch;
mod prelude;
mod error;

use prelude::*;

use axum::{Extension, Router};
use axum::extract::{Path, State, Json};
use axum::routing::put;

use eyre::WrapErr;


use sqlx::MySqlPool;

use patch::{TabelaSocioDep,SocioDepPatch};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")
        .wrap_err("DATABASE_URL must be set")?;

    let db = MySqlPool::connect(&database_url).await?;

    let app = Router::new().route("/sociodep/:id", put(sociodep_patch)).layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn sociodep_patch(db: Extension<MySqlPool>, Path(id): Path<String>, Json(payload): Json<SocioDepPatch>) -> Result<Json<TabelaSocioDep>> where  Result<Json<TabelaSocioDep>>: axum::response::IntoResponse {
    let mut socio = dbg!(TabelaSocioDep::read(&*db, id).await?);

    let payload = dbg!(payload);

    socio.patch(payload);

    dbg!(&socio);

    socio.write(&*db).await?;

    Ok(Json(socio))
}