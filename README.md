## Configuração do Banco de Dados

Para configurar o banco de dados, altere o arquivo `.env` na raiz do projeto com as seguintes variáveis:

```plaintext
DATABASE_URL=mysql://user:password@localhost/nome_do_banco
```

## Para gerar a documentação

```
cargo doc --open --no-deps
```

## Para usar o programa

```
cargo run --release -- --porta 3048
```

Execute o servidor na porta 3048 ou em outra porta de sua escolha, acesse a rota GET sociodep ou GET sociodep/{:id}

Outras rotas liberadas

```bash
POST login #gera um novo token para o usuário 
GET sociodep #lista todos os sócios dependentes
GET sociodep/{:id} #lista apenas um sócio dependente com id específico
POST sociodep body:{} #insere um sócio dependente com base no json anexado
PUT sociodep/{:id} body:{} #altera um sócio dependente com id específico, modifica apenas os campos informados
```

## Sobre a autenticação

deve informar o token em cada rota no formato bearer Exemplo `"authorization": "Bearer SEU_TOKEN_AQUI"`
