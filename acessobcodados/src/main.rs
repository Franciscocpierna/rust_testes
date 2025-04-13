
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Conecta ao banco de dados contaspagar.db
    let conn = Connection::open("contaspagar.db")?;

    println!("Conexão com o banco de dados estabelecida com sucesso!");

    // Feche a conexão automaticamente ao final do escopo
    Ok(())
}