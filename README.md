# Cancellazione Database MongoDB

Questo programma scritto in Rust si connette a un server MongoDB e cancella un database specificato dall'utente tramite un argomento della riga di comando.

## Requisiti

- Rust
- Cargo
- Un'istanza di MongoDB in esecuzione su `localhost:27017`

## Utilizzo

1. Clona il repository o copia il codice sorgente nel tuo ambiente di sviluppo.
2. Aggiungi le dipendenze necessarie nel file `Cargo.toml`:
    ```toml
    [dependencies]
    mongodb = "2.0"
    tokio = { version = "1", features = ["full"] }
    ```
3. Compila ed esegui il programma passando il nome del database da cancellare come argomento:
    ```sh
    cargo run -- <nome_database>
    ```

    Ad esempio, per cancellare un database chiamato `Xlase9`:
    ```sh
    cargo run -- myDB
    ```

## Funzionamento

Il programma esegue i seguenti passaggi:
1. Recupera gli argomenti dalla riga di comando.
2. Verifica che sia stato specificato un nome di database.
3. Si connette al server MongoDB in esecuzione su [`localhost:27017`](command:_github.copilot.openSymbolFromReferences?%5B%22localhost%3A27017%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22c%3A%5C%5CUsers%5C%5CGiulio_Marchesini%5C%5CDesktop%5C%5Cdev%5C%5Crust%5C%5Ccancella_db%5C%5Csrc%5C%5Cmain.rs%22%2C%22_sep%22%3A1%2C%22external%22%3A%22file%3A%2F%2F%2Fc%253A%2FUsers%2FGiulio_Marchesini%2FDesktop%2Fdev%2Frust%2Fcancella_db%2Fsrc%2Fmain.rs%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FGiulio_Marchesini%2FDesktop%2Fdev%2Frust%2Fcancella_db%2Fsrc%2Fmain.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A15%2C%22character%22%3A57%7D%7D%5D%5D "Go to definition").
4. Cancella il database specificato dall'argomento.
5. Stampa un messaggio di conferma.

## Note

Assicurati che il server MongoDB sia in esecuzione e accessibile all'indirizzo [`localhost:27017`](command:_github.copilot.openSymbolFromReferences?%5B%22localhost%3A27017%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22c%3A%5C%5CUsers%5C%5CGiulio_Marchesini%5C%5CDesktop%5C%5Cdev%5C%5Crust%5C%5Ccancella_db%5C%5Csrc%5C%5Cmain.rs%22%2C%22_sep%22%3A1%2C%22external%22%3A%22file%3A%2F%2F%2Fc%253A%2FUsers%2FGiulio_Marchesini%2FDesktop%2Fdev%2Frust%2Fcancella_db%2Fsrc%2Fmain.rs%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FGiulio_Marchesini%2FDesktop%2Fdev%2Frust%2Fcancella_db%2Fsrc%2Fmain.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A15%2C%22character%22%3A57%7D%7D%5D%5D "Go to definition") prima di eseguire il programma.