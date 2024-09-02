use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Recupera gli argomenti dalla riga di comando
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("nessun argomento specificato. inserire il nome del database dopo il nome del programma");
        std::process::exit(1);
    }
    let db_name = &args[1];

    // Configura le opzioni del client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Crea un nuovo client
    let client = Client::with_options(client_options)?;

    // Cancella il database specificato dall'argomento
    client.database(db_name).drop().await?;

    println!("Database '{}' cancellato con successo!", db_name);

    Ok(())
}
