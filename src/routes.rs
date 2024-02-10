use crate::auth::{authorize, AuthPayload, AuthenticatedUser};
use crate::prelude::*;
use crate::entidade::{SelectSocioDep, MergeSocioDep, InsertSocioDep};
use axum::extract::{Path, Json};
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::MySqlPool;

use axum::debug_handler;

/// Efetua login no sistema.
///
/// Esta rota gera um novo token JWT com validade de 24 horas. O usuário deve fornecer
/// credenciais válidas no corpo da requisição para receber um token.
#[debug_handler]
pub async fn login(payload: Json<AuthPayload>) -> impl IntoResponse {
    authorize(payload).await.into_response()
}


/// Atualiza um registro de sócio dependente pelo ID.
///
/// Esta rota permite a atualização dos dados de um sócio dependente específico,
/// identificado pelo ID. O corpo da requisição deve conter os dados atualizados.
#[debug_handler]
pub async fn sociodep_put(db: Extension<MySqlPool>, Path(id): Path<i32>, user: AuthenticatedUser, Json(payload): Json<MergeSocioDep>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let mut socio = dbg!(SelectSocioDep::read(&db, id).await?);
    let payload = dbg!(payload);

    socio.patch(payload);

    dbg!(&socio);

    socio.update(&db).await?;

    Ok(Json(socio))
}

/// Insere um novo registro de sócio dependente.
///
/// Esta rota permite a criação de um novo sócio dependente no banco de dados.
/// O corpo da requisição deve conter os dados necessários para a criação do registro.

#[debug_handler]
pub async fn sociodep_post(db: Extension<MySqlPool>, user: AuthenticatedUser, Json(payload): Json<InsertSocioDep>) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let payload = dbg!(payload);
    let socio = dbg!(InsertSocioDep::create(&payload, &db).await?);

    dbg!(&socio);

    Ok(Json(socio))
}

/// Recupera um registro de sócio dependente pelo ID.
///
/// Esta rota retorna os detalhes de um sócio dependente específico, identificado pelo ID.
/// O ID deve ser passado como parte do caminho da URL.
pub async fn sociodep_get(db: Extension<MySqlPool>, Path(id): Path<i32>, user: AuthenticatedUser) -> Result<Json<SelectSocioDep>> where  Result<Json<SelectSocioDep>>: axum::response::IntoResponse {
    let socio = dbg!(SelectSocioDep::read(&db, id).await?);
    dbg!(&socio);
    Ok(Json(socio))
}

/// Recupera todos os registros de sócios dependentes.
///
/// Esta rota retorna uma lista com todos os sócios dependentes cadastrados no banco de dados.
pub async fn sociodep_get_all(db: Extension<MySqlPool>, user: AuthenticatedUser) -> Result<Json<Vec<SelectSocioDep>>> where  Result<Json<Vec<SelectSocioDep>>>: axum::response::IntoResponse {
    let socio = dbg!(SelectSocioDep::read_all(&db).await?);
    dbg!(&socio);
    Ok(Json(socio))
}