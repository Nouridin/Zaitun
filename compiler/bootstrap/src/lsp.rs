use lsp_types::{ServerCapabilities, TextDocumentSyncKind};

pub struct ZaitunLanguageServer {
    // ... existing code ...
}

impl ZaitunLanguageServer {
    pub fn new() -> Self {
        ZaitunLanguageServer {
            // Initialize server state
            // ... existing code ...
        }
    }

    pub fn capabilities() -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncKind::INCREMENTAL.into()),
            // Implement documented capabilities
            // ... existing code ...
        }
    }
}