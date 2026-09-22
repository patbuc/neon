// Language Server Protocol implementation for Neon

pub mod backend;
pub mod diagnostics;
pub mod document_store;
pub mod semantic_tokens;

pub use backend::NeonLanguageServer;
pub use diagnostics::{compilation_error_to_diagnostic, generate_diagnostics};
pub use document_store::{Document, DocumentStore};
pub use semantic_tokens::{create_legend, generate_semantic_tokens, token_type_to_semantic_index};

// Public API for initializing and running the LSP server
pub async fn run_lsp_server() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Starting Neon Language Server...");

    // Create stdin/stdout transport for LSP communication
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Create the LSP server with our backend
    let (service, socket) = tower_lsp::LspService::new(NeonLanguageServer::new);

    // Serve requests until shutdown
    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;

    eprintln!("Neon Language Server shutdown complete");

    Ok(())
}
