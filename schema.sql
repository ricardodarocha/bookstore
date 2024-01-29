create database if not exists proto;

use proto;

create table if not exists tabelasociodep(
  id varchar(5),
  titulo VARCHAR(6),
  nome VARCHAR(50),
  tipo VARCHAR(4),
  cpf VARCHAR(14),
  nascimento DATE,
  sexo VARCHAR(9),
  telefone VARCHAR(14),
  codigosocio INT(11) NOT NULL AUTO_INCREMENT,
  senha VARCHAR(50) NOT NULL,
  email VARCHAR(200) NOT NULL,
  statussocio VARCHAR(100) NOT NULL,
  PRIMARY KEY (CODIGOSOCIO)
);

