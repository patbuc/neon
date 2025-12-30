// Language Server Protocol implementation for Neon

pub mod backend;
pub mod handlers;
pub mod diagnostics;
pub mod document_store;
pub mod semantic_tokens;

pub use backend::NeonLanguageServer;
pub use document_store::{Document, DocumentStore};
pub use semantic_tokens::{create_legend, generate_semantic_tokens, token_type_to_semantic_index};
pub use diagnostics::{compilation_error_to_diagnostic, generate_diagnostics};

// Public API for initializing and running the LSP server
pub async fn run_lsp_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging to stderr only (stdout is reserved for LSP protocol)
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();

    eprintln!("Starting Neon Language Server...");

    // Create stdin/stdout transport for LSP communication
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Create the LSP server with our backend
    let (service, socket) = tower_lsp::LspService::new(|client| {
        NeonLanguageServer::new(client)
    });

    // Serve requests until shutdown
    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;

    eprintln!("Neon Language Server shutdown complete");

    Ok(())
}
