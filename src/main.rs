#![allow(unused)]

mod routes;
mod auth;
mod args;
mod entidade;
mod prelude;
mod error;

use args::AppConfig;
use clap::Parser;
use axum::{Extension, Router, routing::{get, put, post}, };//State,
use eyre::WrapErr;
use sqlx::MySqlPool;
use routes::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")
        .wrap_err("DATABASE_URL must be set")?;

    let db = MySqlPool::connect(&database_url).await?;

    let app = Router::new()
    .route("/login", post(login)) //Gera um novo token com prazo de 24hs
    .route("/sociodep", post(sociodep_post)) //Insert 
    .route("/sociodep", get(sociodep_get_all)) //Select all
    .route("/sociodep/:id", get(sociodep_get)) //Select one
    // .route("/sociodep/:id", put(sociodep_put)) //Update
    .layer(Extension(db));


    let config = AppConfig::parse();
    let addr = format!("0.0.0.0:{}", config.porta);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
