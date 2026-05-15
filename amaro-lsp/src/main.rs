mod ast;
mod info;
mod parser;
mod server;

use std::io::Write;
use std::{fs::OpenOptions, panic};

use server::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    panic::set_hook(Box::new(|panic_info| {
        // Open or create the log file
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("panic.log")
            .expect("Failed to open panic.log");

        // Write the panic message
        writeln!(file, "Panic occurred: {:?}", panic_info).expect("Failed to write to panic.log");

        // Optional: Also print to stderr
        println!("Panic occurred! See panic.log for details.");
    }));

    // let (service, socket) = LspService::new(|client| Backend::new(client));
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
