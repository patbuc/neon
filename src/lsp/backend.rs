// LSP backend implementation
//
// Implements the tower_lsp::LanguageServer trait to handle LSP protocol messages.
// Integrates document store, semantic tokens, and diagnostics.

use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::lsp::document_store::DocumentStore;
use crate::lsp::semantic_tokens::{create_legend, generate_semantic_tokens};
use crate::lsp::diagnostics::generate_diagnostics;

/// Neon language server backend
///
/// Manages document synchronization, semantic tokens, and diagnostics
/// for the Neon programming language.
#[derive(Debug)]
pub struct NeonLanguageServer {
    /// LSP client handle for sending notifications/requests to the client
    client: Client,
    /// Thread-safe document store for managing open files
    document_store: Arc<Mutex<DocumentStore>>,
}

impl NeonLanguageServer {
    /// Create a new Neon language server instance
    pub fn new(client: Client) -> Self {
        Self {
            client,
            document_store: Arc::new(Mutex::new(DocumentStore::new())),
        }
    }

    /// Get a reference to the document store (for testing/debugging)
    pub fn document_store(&self) -> Arc<Mutex<DocumentStore>> {
        Arc::clone(&self.document_store)
    }

    /// Publish diagnostics for a document
    ///
    /// Compiles the document source and publishes any errors as LSP diagnostics.
    /// If the document is not found, logs an error and does nothing.
    async fn publish_diagnostics_for_document(&self, uri: Url) {
        let store = self.document_store.lock().await;

        match store.get(&uri) {
            Some(document) => {
                let diagnostics = generate_diagnostics(&document.text);
                let version = Some(document.version);
                drop(store); // Release lock before async call

                self.client
                    .publish_diagnostics(uri, diagnostics, version)
                    .await;
            }
            None => {
                eprintln!("Warning: Cannot publish diagnostics for unknown document: {}", uri);
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for NeonLanguageServer {
    /// Initialize the language server with client capabilities
    ///
    /// Returns server capabilities including:
    /// - Full document synchronization
    /// - Semantic tokens support
    /// - Diagnostic publishing
    async fn initialize(&self, _params: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // Full document synchronization (client sends entire document on change)
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),

                // Semantic tokens support for syntax highlighting
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: None,
                            },
                            legend: create_legend(),
                            range: None,
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                    ),
                ),

                // Other capabilities disabled for now
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "neon-language-server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    /// Called after initialization is complete
    ///
    /// Logs a message indicating the server is ready.
    async fn initialized(&self, _params: InitializedParams) {
        eprintln!("Neon language server initialized and ready");
    }

    /// Shutdown the server
    ///
    /// Currently a no-op as we don't have cleanup tasks.
    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    /// Handle document open notification
    ///
    /// Adds the document to the store and publishes initial diagnostics.
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;
        let version = params.text_document.version;

        eprintln!("Document opened: {}", uri);

        // Add document to store
        {
            let mut store = self.document_store.lock().await;
            store.insert(uri.clone(), text, version);
        }

        // Publish initial diagnostics
        self.publish_diagnostics_for_document(uri).await;
    }

    /// Handle document change notification
    ///
    /// Updates the document in the store and re-publishes diagnostics.
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;

        // Full document sync: take the last change which contains the entire document
        if let Some(change) = params.content_changes.into_iter().last() {
            eprintln!("Document changed: {} (version {})", uri, version);

            // Update document in store
            {
                let mut store = self.document_store.lock().await;
                if let Err(e) = store.update(&uri, change.text, version) {
                    eprintln!("Error updating document: {}", e);
                    return;
                }
            }

            // Re-publish diagnostics
            self.publish_diagnostics_for_document(uri).await;
        }
    }

    /// Handle document close notification
    ///
    /// Removes the document from the store and clears diagnostics.
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        eprintln!("Document closed: {}", uri);

        // Remove document from store
        {
            let mut store = self.document_store.lock().await;
            store.remove(&uri);
        }

        // Clear diagnostics for closed document
        self.client
            .publish_diagnostics(uri, vec![], None)
            .await;
    }

    /// Handle semantic tokens request
    ///
    /// Generates and returns semantic tokens for the entire document.
    /// Returns None if the document is not found.
    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;

        eprintln!("Semantic tokens requested: {}", uri);

        let store = self.document_store.lock().await;

        match store.get(&uri) {
            Some(document) => {
                let tokens = generate_semantic_tokens(&document.text);

                Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                    result_id: None,
                    data: tokens,
                })))
            }
            None => {
                eprintln!("Warning: Semantic tokens requested for unknown document: {}", uri);
                Ok(None)
            }
        }
    }
}
