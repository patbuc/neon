// Document storage for LSP server
// Tracks open files, their content, and version numbers

use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

/// Represents a single document in the LSP server
#[derive(Debug, Clone)]
pub struct Document {
    /// URI of the document
    pub uri: Url,
    /// Full text content of the document
    pub text: String,
    /// Version number (incremented on each update)
    pub version: i32,
}

impl Document {
    /// Create a new document
    pub fn new(uri: Url, text: String, version: i32) -> Self {
        Self { uri, text, version }
    }
}

/// Thread-safe document store for managing open files
///
/// Uses full document synchronization (TextDocumentSyncKind::FULL).
/// Should be wrapped in Arc<Mutex<>> for concurrent access.
#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: HashMap<Url, Document>,
}

impl DocumentStore {
    /// Create a new empty document store
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Insert or replace a document in the store
    pub fn insert(&mut self, uri: Url, text: String, version: i32) {
        let document = Document::new(uri.clone(), text, version);
        self.documents.insert(uri, document);
    }

    /// Get a reference to a document by URI
    pub fn get(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }

    /// Get a mutable reference to a document by URI
    pub fn get_mut(&mut self, uri: &Url) -> Option<&mut Document> {
        self.documents.get_mut(uri)
    }

    /// Remove a document from the store
    /// Returns the removed document if it existed
    pub fn remove(&mut self, uri: &Url) -> Option<Document> {
        self.documents.remove(uri)
    }

    /// Update an existing document with new content and version
    /// Returns Ok(()) if the document was updated, Err if the document doesn't exist
    pub fn update(&mut self, uri: &Url, text: String, version: i32) -> Result<(), String> {
        match self.documents.get_mut(uri) {
            Some(doc) => {
                doc.text = text;
                doc.version = version;
                Ok(())
            }
            None => Err(format!("Document not found: {}", uri)),
        }
    }

    /// Get the number of documents in the store
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Check if the store is empty
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    /// Check if a document exists in the store
    pub fn contains(&self, uri: &Url) -> bool {
        self.documents.contains_key(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_url(path: &str) -> Url {
        Url::parse(&format!("file:///{}", path)).unwrap()
    }

    #[test]
    fn test_new_store_is_empty() {
        let store = DocumentStore::new();
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn test_insert_document() {
        let mut store = DocumentStore::new();
        let uri = test_url("test.neon");
        let text = "var x = 10".to_string();

        store.insert(uri.clone(), text.clone(), 1);

        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());
        assert!(store.contains(&uri));

        let doc = store.get(&uri).unwrap();
        assert_eq!(doc.uri, uri);
        assert_eq!(doc.text, text);
        assert_eq!(doc.version, 1);
    }

    #[test]
    fn test_get_nonexistent_document() {
        let store = DocumentStore::new();
        let uri = test_url("nonexistent.neon");

        assert!(store.get(&uri).is_none());
        assert!(!store.contains(&uri));
    }

    #[test]
    fn test_update_document() {
        let mut store = DocumentStore::new();
        let uri = test_url("test.neon");

        // Insert initial document
        store.insert(uri.clone(), "var x = 10".to_string(), 1);

        // Update the document
        let new_text = "var x = 20".to_string();
        let result = store.update(&uri, new_text.clone(), 2);

        assert!(result.is_ok());

        let doc = store.get(&uri).unwrap();
        assert_eq!(doc.text, new_text);
        assert_eq!(doc.version, 2);
    }

    #[test]
    fn test_update_nonexistent_document() {
        let mut store = DocumentStore::new();
        let uri = test_url("nonexistent.neon");

        let result = store.update(&uri, "text".to_string(), 1);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Document not found"));
    }

    #[test]
    fn test_remove_document() {
        let mut store = DocumentStore::new();
        let uri = test_url("test.neon");

        store.insert(uri.clone(), "var x = 10".to_string(), 1);
        assert_eq!(store.len(), 1);

        let removed = store.remove(&uri);
        assert!(removed.is_some());
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
        assert!(!store.contains(&uri));

        let removed_doc = removed.unwrap();
        assert_eq!(removed_doc.uri, uri);
        assert_eq!(removed_doc.text, "var x = 10");
        assert_eq!(removed_doc.version, 1);
    }

    #[test]
    fn test_remove_nonexistent_document() {
        let mut store = DocumentStore::new();
        let uri = test_url("nonexistent.neon");

        let removed = store.remove(&uri);
        assert!(removed.is_none());
    }

    #[test]
    fn test_multiple_documents() {
        let mut store = DocumentStore::new();

        let uri1 = test_url("file1.neon");
        let uri2 = test_url("file2.neon");
        let uri3 = test_url("file3.neon");

        store.insert(uri1.clone(), "content1".to_string(), 1);
        store.insert(uri2.clone(), "content2".to_string(), 1);
        store.insert(uri3.clone(), "content3".to_string(), 1);

        assert_eq!(store.len(), 3);
        assert!(store.contains(&uri1));
        assert!(store.contains(&uri2));
        assert!(store.contains(&uri3));

        // Update one
        store.update(&uri2, "updated content2".to_string(), 2).unwrap();
        assert_eq!(store.get(&uri2).unwrap().version, 2);

        // Remove one
        store.remove(&uri1);
        assert_eq!(store.len(), 2);
        assert!(!store.contains(&uri1));
    }

    #[test]
    fn test_version_tracking() {
        let mut store = DocumentStore::new();
        let uri = test_url("test.neon");

        // Insert with version 1
        store.insert(uri.clone(), "v1".to_string(), 1);
        assert_eq!(store.get(&uri).unwrap().version, 1);

        // Update to version 2
        store.update(&uri, "v2".to_string(), 2).unwrap();
        assert_eq!(store.get(&uri).unwrap().version, 2);

        // Update to version 3
        store.update(&uri, "v3".to_string(), 3).unwrap();
        assert_eq!(store.get(&uri).unwrap().version, 3);
    }

    #[test]
    fn test_insert_replaces_existing() {
        let mut store = DocumentStore::new();
        let uri = test_url("test.neon");

        store.insert(uri.clone(), "original".to_string(), 1);
        assert_eq!(store.len(), 1);

        // Insert again with same URI - should replace
        store.insert(uri.clone(), "replaced".to_string(), 2);
        assert_eq!(store.len(), 1); // Still only one document

        let doc = store.get(&uri).unwrap();
        assert_eq!(doc.text, "replaced");
        assert_eq!(doc.version, 2);
    }
}
