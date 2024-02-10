//! # API de Conexão com Banco de Dados Summer na Nuvem
//! 
//! # summerapi
//!
//! Este programa implementa uma API para conexão com o banco de dados Summer na Nuvem
//! via driver MySQL. Ele oferece operações CRUD (Create, Read, Update, Delete) para a
//! tabela `sociodep`, permitindo a manipulação de registros de sócios dependentes.
//! 
//! ## Versão
//! 
//! ```
//! 0.1.1
//! ```
//! ## Funcionalidades
//!
//! - **Criação de Registros**: Permite inserir novos sócios dependentes no banco de dados.
//! - **Leitura de Registros**: Suporta a consulta de registros existentes, tanto de um
//!   único sócio dependente pelo ID quanto de todos os sócios dependentes.
//! - **Atualização de Registros**: Oferece a capacidade de atualizar os dados de um
//!   sócio dependente específico.
//! 
//! login	Efetua login no sistema.
//! sociodep_get	Recupera um registro de sócio dependente pelo ID.
//! sociodep_get_all	Recupera todos os registros de sócios dependentes.
//! sociodep_post	Insere um novo registro de sócio dependente.
//! sociodep_put	Atualiza um registro de sócio dependente pelo ID.
//!
//! ## Models
//! 
//! Structs
//! InsertSocioDep	struct que representa um JSON vindo de uma requisição que adicionia um registro na tabela SocioDep. Valores None representam campos que não estão sendo modificados pela requisição.
//! MergeSocioDep	struct que representa um JSON vindo de uma requisição que modifica determinada linha de SocioDep. Valores None representam campos que não estão sendo modificados pela requisição.
//! SelectSocioDep  struct que representa uma linha da tabela SocioDep
//! 
//! ## Como Usar
//!
//! A API está estruturada em torno de rotas HTTP que correspondem às operações CRUD.
//! Para mais detalhes sobre como interagir com cada rota, consulte a documentação específica
//! de cada função de operação no código.
//!
//! ## Configuração
//!
//! Antes de executar a API, é necessário configurar a conexão com o banco de dados Summer
//! na Nuvem. Isso é feito ajustando as configurações no arquivo `.env` para corresponder
//! ao seu ambiente de banco de dados.

#![doc = include_str!("../README.md")]

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
    .route("/sociodep/:id", put(sociodep_put)) //Update
    .layer(Extension(db));


    let config = AppConfig::parse();
    let addr = format!("0.0.0.0:{}", config.porta);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
