pub use eyre::{bail,Result,WrapErr};

use crate::prelude::*;

use time::Date;
use serde::{Deserialize,Serialize};

use sqlx::MySqlPool;

// Representa uma linha da tabelasociodep do banco de dados. Valores `None`
// representam um valor null do SQL.
#[derive(Debug, Serialize, Deserialize)]
pub struct TabelaSocioDep {
  id: Option<String>,
  titulo: Option<String>,
  nome: Option<String>,
  tipo: Option<String>,
  cpf: Option<String>,
  nascimento: Option<Date>,
  sexo: Option<String>,
  telefone: Option<String>,

  #[serde(skip_serializing)]
  /// Não é serializado para não vazar as chaves primárias do db (toda interação
  /// externa é feita pelo id)
  codigosocio: i32,

  senha: String,
  email: String,
  statussocio: String,
}

/// struct que representa um JSON vindo de uma requisição que modifica
/// determinada linha de `TabelaSocioDep`. Valores `None` representam campos que
/// não estão sendo modificados pela requisição.
///
/// Se supõe que os campos id e codigosocio não serão mudados através dessa requisição.
#[derive(Debug, Deserialize, Serialize)]
pub struct SocioDepPatch {
  titulo: Option<String>,
  nome: Option<String>,
  tipo: Option<String>,
  cpf: Option<String>,
  nascimento: Option<Date>,
  sexo: Option<String>,
  telefone: Option<String>,
  senha: Option<String>,
  email: Option<String>,
  statussocio: Option<String>,
}

impl TabelaSocioDep {
    pub async fn read(db: &MySqlPool, id: String) -> Result<Self> {
        let tabela = sqlx::query_as!(TabelaSocioDep, "select * from tabelasociodep where id = ?", id).fetch_one(db).await?;

        Ok(tabela)
    }

    pub async fn write(&self, db: &MySqlPool) -> Result<()> {
        let linhas = sqlx::query!("update tabelasociodep set
            titulo = ?,
            nome = ?,
            tipo = ?,
            cpf = ?,
            nascimento = ?,
            sexo = ?,
            telefone = ?,
            senha = ?,
            email = ?,
            statussocio = ?

            where id = ?",
            self.titulo,
            self.nome,
            self.tipo,
            self.cpf,
            self.nascimento,
            self.sexo,
            self.telefone,
            self.senha,
            self.email,
            self.statussocio,

            self.id,
        ).execute(db).await?
        .rows_affected();

        if linhas != 1 {
          bail!("nothing was updated at id {:?}", self.id);
        }

        Ok(())
    }

    pub fn patch(&mut self, json: SocioDepPatch) {
          self.titulo = json.titulo.or(self.titulo.clone());
          self.nome = json.nome.or(self.nome.clone());
          self.tipo = json.tipo.or(self.tipo.clone());
          self.cpf = json.cpf.or(self.cpf.clone());
          self.nascimento = json.nascimento.or(self.nascimento.clone());
          self.sexo = json.sexo.or(self.sexo.clone());
          self.telefone = json.telefone.or(self.telefone.clone());
          self.senha = json.senha.unwrap_or(self.senha.clone());
          self.email = json.email.unwrap_or(self.email.clone());
          self.statussocio = json.statussocio.unwrap_or(self.statussocio.clone());
    }
}