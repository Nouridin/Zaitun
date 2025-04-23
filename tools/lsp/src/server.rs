use lsp_types::{
    CompletionOptions, InitializeParams, InitializeResult, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::io;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde_json::Value;

// LSP message types
#[derive(Debug, Serialize, Deserialize)]
struct InitializeParams {
    capabilities: ClientCapabilities,
    #[serde(rename = "rootUri")]
    root_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientCapabilities {
    #[serde(rename = "textDocument")]
    text_document: Option<TextDocumentClientCapabilities>,
    workspace: Option<WorkspaceClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextDocumentClientCapabilities {
    completion: Option<CompletionClientCapabilities>,
    hover: Option<HoverClientCapabilities>,
    #[serde(rename = "documentSymbol")]
    document_symbol: Option<DocumentSymbolClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompletionClientCapabilities {
    #[serde(rename = "dynamicRegistration")]
    dynamic_registration: Option<bool>,
    #[serde(rename = "completionItem")]
    completion_item: Option<CompletionItemCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompletionItemCapabilities {
    #[serde(rename = "snippetSupport")]
    snippet_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HoverClientCapabilities {
    #[serde(rename = "dynamicRegistration")]
    dynamic_registration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentSymbolClientCapabilities {
    #[serde(rename = "dynamicRegistration")]
    dynamic_registration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceClientCapabilities {
    #[serde(rename = "applyEdit")]
    apply_edit: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitializeResult {
    capabilities: ServerCapabilities,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerCapabilities {
    #[serde(rename = "textDocumentSync")]
    text_document_sync: TextDocumentSyncOptions,
    #[serde(rename = "completionProvider")]
    completion_provider: CompletionOptions,
    #[serde(rename = "hoverProvider")]
    hover_provider: bool,
    #[serde(rename = "documentSymbolProvider")]
    document_symbol_provider: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextDocumentSyncOptions {
    #[serde(rename = "openClose")]
    open_close: bool,
    change: i32, // 1 = full, 2 = incremental
}

#[derive(Debug, Serialize, Deserialize)]
struct CompletionOptions {
    #[serde(rename = "triggerCharacters")]
    trigger_characters: Vec<String>,
    #[serde(rename = "resolveProvider")]
    resolve_provider: bool,
}

// LSP types
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

// Document management
#[derive(Debug, Clone)]
pub struct Document {
    pub uri: String,
    pub version: i32,
    pub text: String,
    pub diagnostics: Vec<Diagnostic>,
}

// Server implementation
pub struct LanguageServer {
    documents: Arc<Mutex<HashMap<String, Document>>>,
    workspace_folders: Arc<Mutex<Vec<String>>>,
    symbol_table: Arc<Mutex<SymbolTable>>,
}

impl LanguageServer {
    pub fn new() -> Self {
        LanguageServer {
            documents: Arc::new(Mutex::new(HashMap::new())),
            workspace_folders: Arc::new(Mutex::new(Vec::new())),
            symbol_table: Arc::new(Mutex::new(SymbolTable::new())),
        }
    }
    
    pub fn initialize(&self, root_uri: Option<String>) -> io::Result<()> {
        if let Some(uri) = root_uri {
            let mut folders = self.workspace_folders.lock().unwrap();
            folders.push(uri);
            
            // Index workspace
            self.index_workspace()?;
        }
        
        Ok(())
    }
    
    pub fn shutdown(&self) -> io::Result<()> {
        // Clean up resources
        self.documents.lock().unwrap().clear();
        self.workspace_folders.lock().unwrap().clear();
        self.symbol_table.lock().unwrap().clear();
        
        Ok(())
    }
    
    pub fn did_open(&self, uri: &str, version: i32, text: &str) -> io::Result<()> {
        let mut documents = self.documents.lock().unwrap();
        
        let document = Document {
            uri: uri.to_string(),
            version,
            text: text.to_string(),
            diagnostics: Vec::new(),
        };
        
        documents.insert(uri.to_string(), document);
        
        // Validate document and update diagnostics
        self.validate_document(uri)?;
        
        Ok(())
    }
    
    pub fn did_change(&self, uri: &str, version: i32, text: &str) -> io::Result<()> {
        let mut documents = self.documents.lock().unwrap();
        
        if let Some(document) = documents.get_mut(uri) {
            document.version = version;
            document.text = text.to_string();
            
            // Validate document and update diagnostics
            drop(documents); // Release lock before calling validate_document
            self.validate_document(uri)?;
        }
        
        Ok(())
    }
    
    pub fn did_close(&self, uri: &str) -> io::Result<()> {
        let mut documents = self.documents.lock().unwrap();
        documents.remove(uri);
        
        Ok(())
    }
    
    pub fn completion(&self, uri: &str, position: Position) -> io::Result<Vec<CompletionItem>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get context at position
            let context = self.get_completion_context(document, position)?;
            
            // Generate completion items based on context
            let mut items = Vec::new();
            
            // Add keywords
            for keyword in KEYWORDS {
                items.push(CompletionItem {
                    label: keyword.to_string(),
                    kind: CompletionItemKind::Keyword,
                    detail: Some("Keyword".to_string()),
                    documentation: None,
                    insert_text: Some(keyword.to_string()),
                });
            }
            
            // Add symbols from symbol table
            for symbol in symbol_table.get_symbols() {
                let kind = match symbol.kind {
                    SymbolKind::Class => CompletionItemKind::Class,
                    SymbolKind::Function => CompletionItemKind::Function,
                    SymbolKind::Variable => CompletionItemKind::Variable,
                    SymbolKind::Struct => CompletionItemKind::Struct,
                    SymbolKind::Enum => CompletionItemKind::Enum,
                    SymbolKind::Interface => CompletionItemKind::Interface,
                    SymbolKind::Module => CompletionItemKind::Module,
                };
                
                items.push(CompletionItem {
                    label: symbol.name.clone(),
                    kind,
                    detail: Some(format!("{:?}", symbol.kind)),
                    documentation: symbol.documentation.clone(),
                    insert_text: Some(symbol.name.clone()),
                });
            }
            
            Ok(items)
        } else {
            Ok(Vec::new())
        }
    }
    
    pub fn definition(&self, uri: &str, position: Position) -> io::Result<Option<Location>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get word at position
            let word = self.get_word_at_position(document, position)?;
            
            // Look up symbol definition
            if let Some(symbol) = symbol_table.find_symbol(&word) {
                return Ok(Some(Location {
                    uri: symbol.location.uri.clone(),
                    range: symbol.location.range.clone(),
                }));
            }
        }
        
        Ok(None)
    }
    
    pub fn hover(&self, uri: &str, position: Position) -> io::Result<Option<String>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get word at position
            let word = self.get_word_at_position(document, position)?;
            
            // Look up symbol information
            if let Some(symbol) = symbol_table.find_symbol(&word) {
                let mut hover_text = format!("**{}** ({})\n\n", symbol.name, format!("{:?}", symbol.kind));
                
                if let Some(doc) = &symbol.documentation {
                    hover_text.push_str(doc);
                }
                
                return Ok(Some(hover_text));
            }
        }
        
        Ok(None)
    }
    
    pub fn references(&self, uri: &str, position: Position) -> io::Result<Vec<Location>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get word at position
            let word = self.get_word_at_position(document, position)?;
            
            // Find all references to the symbol
            let references = symbol_table.find_references(&word);
            
            Ok(references)
        } else {
            Ok(Vec::new())
        }
    }
    
    fn validate_document(&self, uri: &str) -> io::Result<()> {
        let mut documents = self.documents.lock().unwrap();
        
        if let Some(document) = documents.get_mut(uri) {
            // Clear existing diagnostics
            document.diagnostics.clear();
            
            // Parse document and collect diagnostics
            let diagnostics = self.parse_and_validate(&document.text)?;
            document.diagnostics = diagnostics;
            
            // Update symbol table
            drop(documents); // Release lock before updating symbol table
            self.update_symbols(uri)?;
        }
        
        Ok(())
    }
    
    fn parse_and_validate(&self, text: &str) -> io::Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        // Simple validation: check for unmatched braces
        let mut brace_stack = Vec::new();
        let mut line = 0;
        let mut char_pos = 0;
        
        for (i, c) in text.chars().enumerate() {
            if c == '\n' {
                line += 1;
                char_pos = 0;
            } else {
                char_pos += 1;
            }
            
            if c == '{' || c == '(' || c == '[' {
                brace_stack.push((c, line, char_pos));
            } else if c == '}' || c == ')' || c == ']' {
                let matching = match c {
                    '}' => '{',
                    ')' => '(',
                    ']' => '[',
                    _ => unreachable!(),
                };
                
                if let Some((brace, brace_line, brace_char)) = brace_stack.pop() {
                    if brace != matching {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line as u32,
                                    character: char_pos as u32,
                                },
                                end: Position {
                                    line: line as u32,
                                    character: (char_pos + 1) as u32,
                                },
                            },
                            severity: DiagnosticSeverity::Error,
                            message: format!("Mismatched brace: expected closing for '{}'", brace),
                            source: "safelang-lsp".to_string(),
                        });
                    }
                } else {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line as u32,
                                character: char_pos as u32,
                            },
                            end: Position {
                                line: line as u32,
                                character: (char_pos + 1) as u32,
                            },
                        },
                        severity: DiagnosticSeverity::Error,
                        message: format!("Unexpected closing brace '{}'", c),
                        source: "safelang-lsp".to_string(),
                    });
                }
            }
        }
        
        // Report any unclosed braces
        for (brace, line, char_pos) in brace_stack {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line as u32,
                        character: char_pos as u32,
                    },
                    end: Position {
                        line: line as u32,
                        character: (char_pos + 1) as u32,
                    },
                },
                severity: DiagnosticSeverity::Error,
                message: format!("Unclosed brace '{}'", brace),
                source: "safelang-lsp".to_string(),
            });
        }
        
        Ok(diagnostics)
    }
    
    fn index_workspace(&self) -> io::Result<()> {
        let folders = self.workspace_folders.lock().unwrap();
        let mut symbol_table = self.symbol_table.lock().unwrap();
        
        for folder in &*folders {
            let path = Path::new(folder.strip_prefix("file://").unwrap_or(folder));
            self.index_directory(&mut symbol_table, path)?;
        }
        
        Ok(())
    }
    
    fn index_directory(&self, symbol_table: &mut SymbolTable, dir: &Path) -> io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.index_directory(symbol_table, &path)?;
            } else if let Some(ext) = path.extension() {
                if ext == "safe" {
                    self.index_file(symbol_table, &path)?;
                }
            }
        }
        
        Ok(())
    }
}

fn parse_document(text: &str) -> Result<AST, ParseError> {
    // Parse document into AST
    // ... implementation details ...
    Ok(AST::default())
}

#[derive(Default)]
struct AST {
    // AST structure
    // ... implementation details ...
}

struct ParseError {
    // Parse error details
    // ... implementation details ...
}

// LSP message types
#[derive(Debug, Serialize, Deserialize)]
struct RequestMessage {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMessage {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<ResponseError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseError {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NotificationMessage {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
}

// LSP server implementation
pub struct LspServer {
    capabilities: Value,
    documents: Arc<Mutex<HashMap<String, String>>>,
    diagnostics: Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Diagnostic {
    range: Range,
    severity: Option<i32>,
    code: Option<Value>,
    source: Option<String>,
    message: String,
    related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticRelatedInformation {
    location: Location,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Location {
    uri: String,
    range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Range {
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Position {
    line: u32,
    character: u32,
}

impl LspServer {
    pub fn new() -> Self {
        LspServer {
            capabilities: json!({
                "textDocumentSync": 1,  // Full sync
                "completionProvider": {
                    "resolveProvider": true,
                    "triggerCharacters": [".", "::"]
                },
                "hoverProvider": true,
                "definitionProvider": true,
                "referencesProvider": true,
                "documentSymbolProvider": true,
                "workspaceSymbolProvider": true,
                "codeActionProvider": true,
                "documentFormattingProvider": true,
            }),
            documents: Arc::new(Mutex::new(HashMap::new())),
            diagnostics: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn run<R, W>(&self, reader: R, mut writer: W) 
    where
        R: BufRead,
        W: Write,
    {
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => break,
            };
            
            // Parse Content-Length header
            if !line.starts_with("Content-Length:") {
                continue;
            }
            
            let content_length: usize = line["Content-Length:".len()..]
                .trim()
                .parse()
                .unwrap_or(0);
            
            // Skip the empty line
            let mut buffer = String::new();
            if reader.read_line(&mut buffer).is_err() {
                break;
            }
            
            // Read the message content
            let mut content = vec![0; content_length];
            if reader.read_exact(&mut content).is_err() {
                break;
            }
            
            let content = match String::from_utf8(content) {
                Ok(content) => content,
                Err(_) => continue,
            };
            
            // Parse and handle the message
            if let Ok(request) = serde_json::from_str::<RequestMessage>(&content) {
                let response = self.handle_request(request);
                if let Some(response) = response {
                    let response_json = serde_json::to_string(&response).unwrap();
                    let response_message = format!(
                        "Content-Length: {}\r\n\r\n{}",
                        response_json.len(),
                        response_json
                    );
                    if writer.write_all(response_message.as_bytes()).is_err() {
                        break;
                    }
                }
            } else if let Ok(notification) = serde_json::from_str::<NotificationMessage>(&content) {
                self.handle_notification(notification, &mut writer);
            }
        }
    }
    
    fn handle_request(&self, request: RequestMessage) -> Option<ResponseMessage> {
        match request.method.as_str() {
            "initialize" => {
                Some(ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "capabilities": self.capabilities,
                    })),
                    error: None,
                })
            }
            "textDocument/completion" => {
                // Implement completion logic here
                Some(ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!([
                        {
                            "label": "println",
                            "kind": 3
                        }
                    ])),
                    error: None,
                })
            }
            "textDocument/hover" => {
                // Handle hover request
                let response = ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "contents": {
                            "kind": "markdown",
                            "value": "**SafeLang Documentation**\n\nThis is a hover tooltip for the symbol."
                        }
                    })),
                    error: None,
                };
                
                self.send_response(response, writer)?;
            }
            "textDocument/definition" => {
                // Handle go to definition request
                let response = ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct SafeLangDocument {
    uri: Url,
    version: i32,
    content: String,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug)]
struct SafeLangLanguageServer {
    client: Client,
    document_map: Arc<Mutex<HashMap<Url, SafeLangDocument>>>,
}

impl SafeLangLanguageServer {
    fn new(client: Client) -> Self {
        SafeLangLanguageServer {
            client,
            document_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    async fn validate_document(&self, uri: Url) {
        let diagnostics = {
            let documents = self.document_map.lock().unwrap();
            if let Some(document) = documents.get(&uri) {
                self.analyze_document(document)
            } else {
                Vec::new()
            }
        };
        
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
    
    fn analyze_document(&self, document: &SafeLangDocument) -> Vec<Diagnostic> {
        // This is a placeholder for actual language analysis
        // In a real implementation, this would parse the document and report errors
        
        let mut diagnostics = Vec::new();
        
        // Example: Check for unsafe blocks
        for (line_idx, line) in document.content.lines().enumerate() {
            if line.contains("unsafe") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_idx as u32,
                            character: line.find("unsafe").unwrap() as u32,
                        },
                        end: Position {
                            line: line_idx as u32,
                            character: (line.find("unsafe").unwrap() + "unsafe".len()) as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: None,
                    code_description: None,
                    source: Some("safelang".to_string()),
                    message: "Use of unsafe code".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
        
        diagnostics
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for SafeLangLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "SafeLang Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }
    
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "SafeLang Language Server initialized")
            .await;
    }
    
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;
        
        {
            let mut documents = self.document_map.lock().unwrap();
            documents.insert(
                uri.clone(),
                SafeLangDocument {
                    uri: uri.clone(),
                    version,
                    content: text,
                    diagnostics: Vec::new(),
                },
            );
        }
        
        self.validate_document(uri).await;
    }
    
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        
        {
            let mut documents = self.document_map.lock().unwrap();
            if let Some(document) = documents.get_mut(&uri) {
                document.version = version;
                
                // Apply changes
                for change in params.content_changes {
                    if let Some(range) = change.range {
                        // Convert range to string indices
                        let start_line = range.start.line as usize;
                        let start_char = range.start.character as usize;
                        let end_line = range.end.line as usize;
                        let end_char = range.end.character as usize;
                        
                        let lines: Vec<&str> = document.content.lines().collect();
                        let mut new_content = String::new();
                        
                        // Add unchanged lines before the change
                        for i in 0..start_line {
                            new_content.push_str(lines[i]);
                            new_content.push('\n');
                        }
                        
                        // Add the start line up to the change
                        if start_line < lines.len() {
                            new_content.push_str(&lines[start_line][..start_char]);
                        }
                        
                        // Add the changed text
                        new_content.push_str(&change.text);
                        
                        // Add the end line after the change
                        if end_line < lines.len() {
                            new_content.push_str(&lines[end_line][end_char..]);
                            new_content.push('\n');
                        }
                        
                        // Add unchanged lines after the change
                        for i in (end_line + 1)..lines.len() {
                            new_content.push_str(lines[i]);
                            new_content.push('\n');
                        }
                        
                        document.content = new_content;
                    } else {
                        // Full document update
                        document.content = change.text;
                    }
                }
            }
        }
        
        self.validate_document(uri).await;
    }
    
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.validate_document(params.text_document.uri).await;
    }
    
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut documents = self.document_map.lock().unwrap();
        documents.remove(&params.text_document.uri);
        
        // Clear diagnostics for closed file
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
    
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        let documents = self.document_map.lock().unwrap();
        if let Some(document) = documents.get(&uri) {
            // Find the word at the position
            let line_idx = position.line as usize;
            let char_idx = position.character as usize;
            
            if let Some(line) = document.content.lines().nth(line_idx) {
                if char_idx < line.len() {
                    // Find word boundaries
                    let start = line[..char_idx]
                        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    
                    let end = line[char_idx..]
                        .find(|c: char| !c.is_alphanumeric() && c != '_')
                        .map(|i| char_idx + i)
                        .unwrap_or(line.len());
                    
                    if start < end {
                        let word = &line[start..end];
                        
                        // Provide hover information based on the word
                        // This is a placeholder for actual language-specific hover info
                        let hover_text = match word {
                            "let" => "Declares a variable binding",
                            "mut" => "Marks a binding as mutable",
                            "function" => "Declares a function",
                            "class" => "Declares a class",
                            "if" => "Conditional expression",
                            "else" => "Alternative branch of conditional",
                            "for" => "Loop over a collection",
                            "while" => "Conditional loop",
                            "return" => "Return from function",
                            _ => return Ok(None),
                        };
                        
                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: format!("**{}**\n\n{}", word, hover_text),
                            }),
                            range: Some(Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: end as u32,
                                },
                            }),
                        }));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let _uri = params.text_document_position.text_document.uri;
        let _position = params.text_document_position.position;
        
        // This is a placeholder for actual completion logic
        // In a real implementation, this would analyze the document and context
        
        let items = vec![
            CompletionItem {
                label: "function".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Define a function".to_string()),
                insert_text: Some("function ${1:name}(${2:params}) {\n\t${0}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "class".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Define a class".to_string()),
                insert_text: Some("class ${1:Name} {\n\t${0}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Conditional statement".to_string()),
                insert_text: Some("if (${1:condition}) {\n\t${0}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "println".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Keyword".to_string()),
                documentation: None,
                insert_text: Some(keyword.to_string()),
            });
    
    Ok(items)
}
    
    pub fn definition(&self, uri: &str, position: Position) -> io::Result<Option<Location>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get word at position
            let word = self.get_word_at_position(document, position)?;
            
            // Look up symbol definition
            if let Some(symbol) = symbol_table.find_symbol(&word) {
                return Ok(Some(Location {
                    uri: symbol.location.uri.clone(),
                    range: symbol.location.range.clone(),
                }));
            }
        }
        
        Ok(None)
    }
    
    pub fn hover(&self, uri: &str, position: Position) -> io::Result<Option<String>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get word at position
            let word = self.get_word_at_position(document, position)?;
            
            // Look up symbol information
            if let Some(symbol) = symbol_table.find_symbol(&word) {
                let mut hover_text = format!("**{}** ({})\n\n", symbol.name, format!("{:?}", symbol.kind));
                
                if let Some(doc) = &symbol.documentation {
                    hover_text.push_str(doc);
                }
                
                return Ok(Some(hover_text));
            }
        }
        
        Ok(None)
    }
    
    pub fn references(&self, uri: &str, position: Position) -> io::Result<Vec<Location>> {
        let documents = self.documents.lock().unwrap();
        let symbol_table = self.symbol_table.lock().unwrap();
        
        if let Some(document) = documents.get(uri) {
            // Get word at position
            let word = self.get_word_at_position(document, position)?;
            
            // Find all references to the symbol
            let references = symbol_table.find_references(&word);
            
            Ok(references)
        } else {
            Ok(Vec::new())
        }
    }
    
    fn validate_document(&self, uri: &str) -> io::Result<()> {
        let mut documents = self.documents.lock().unwrap();
        
        if let Some(document) = documents.get_mut(uri) {
            // Clear existing diagnostics
            document.diagnostics.clear();
            
            // Parse document and collect diagnostics
            let diagnostics = self.parse_and_validate(&document.text)?;
            document.diagnostics = diagnostics;
            
            // Update symbol table
            drop(documents); // Release lock before updating symbol table
            self.update_symbols(uri)?;
        }
        
        Ok(())
    }
    
    fn parse_and_validate(&self, text: &str) -> io::Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        // Simple validation: check for unmatched braces
        let mut brace_stack = Vec::new();
        let mut line = 0;
        let mut char_pos = 0;
        
        for (i, c) in text.chars().enumerate() {
            if c == '\n' {
                line += 1;
                char_pos = 0;
            } else {
                char_pos += 1;
            }
            
            if c == '{' || c == '(' || c == '[' {
                brace_stack.push((c, line, char_pos));
            } else if c == '}' || c == ')' || c == ']' {
                let matching = match c {
                    '}' => '{',
                    ')' => '(',
                    ']' => '[',
                    _ => unreachable!(),
                };
                
                if let Some((brace, brace_line, brace_char)) = brace_stack.pop() {
                    if brace != matching {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line as u32,
                                    character: char_pos as u32,
                                },
                                end: Position {
                                    line: line as u32,
                                    character: (char_pos + 1) as u32,
                                },
                            },
                            severity: DiagnosticSeverity::Error,
                            message: format!("Mismatched brace: expected closing for '{}'", brace),
                            source: "safelang-lsp".to_string(),
                        });
                    }
                } else {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line as u32,
                                character: char_pos as u32,
                            },
                            end: Position {
                                line: line as u32,
                                character: (char_pos + 1) as u32,
                            },
                        },
                        severity: DiagnosticSeverity::Error,
                        message: format!("Unexpected closing brace '{}'", c),
                        source: "safelang-lsp".to_string(),
                    });
                }
            }
        }
        
        // Report any unclosed braces
        for (brace, line, char_pos) in brace_stack {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line as u32,
                        character: char_pos as u32,
                    },
                    end: Position {
                        line: line as u32,
                        character: (char_pos + 1) as u32,
                    },
                },
                severity: DiagnosticSeverity::Error,
                message: format!("Unclosed brace '{}'", brace),
                source: "safelang-lsp".to_string(),
            });
        }
        
        Ok(diagnostics)
    }
    
    fn index_workspace(&self) -> io::Result<()> {
        let folders = self.workspace_folders.lock().unwrap();
        let mut symbol_table = self.symbol_table.lock().unwrap();
        
        for folder in &*folders {
            let path = Path::new(folder.strip_prefix("file://").unwrap_or(folder));
            self.index_directory(&mut symbol_table, path)?;
        }
        
        Ok(())
    }
    
    fn index_directory(&self, symbol_table: &mut SymbolTable, dir: &Path) -> io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.index_directory(symbol_table, &path)?;
            } else if let Some(ext) = path.extension() {
                if ext == "safe" {
                    self.index_file(symbol_table, &path)?;
                }
            }
        }
        
        Ok(())
    }
}

fn parse_document(text: &str) -> Result<AST, ParseError> {
    // Parse document into AST
    // ... implementation details ...
    Ok(AST::default())
}

#[derive(Default)]
struct AST {
    // AST structure
    // ... implementation details ...
}

struct ParseError {
    // Parse error details
    // ... implementation details ...
}

// LSP message types
#[derive(Debug, Serialize, Deserialize)]
struct RequestMessage {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMessage {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<ResponseError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseError {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NotificationMessage {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
}

// LSP server implementation
pub struct LspServer {
    capabilities: Value,
    documents: Arc<Mutex<HashMap<String, String>>>,
    diagnostics: Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Diagnostic {
    range: Range,
    severity: Option<i32>,
    code: Option<Value>,
    source: Option<String>,
    message: String,
    related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticRelatedInformation {
    location: Location,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Location {
    uri: String,
    range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Range {
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Position {
    line: u32,
    character: u32,
}

impl LspServer {
    pub fn new() -> Self {
        LspServer {
            capabilities: json!({
                "textDocumentSync": 1,  // Full sync
                "completionProvider": {
                    "resolveProvider": true,
                    "triggerCharacters": [".", "::"]
                },
                "hoverProvider": true,
                "definitionProvider": true,
                "referencesProvider": true,
                "documentSymbolProvider": true,
                "workspaceSymbolProvider": true,
                "codeActionProvider": true,
                "documentFormattingProvider": true,
            }),
            documents: Arc::new(Mutex::new(HashMap::new())),
            diagnostics: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn run<R, W>(&self, reader: R, mut writer: W) 
    where
        R: BufRead,
        W: Write,
    {
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => break,
            };
            
            // Parse Content-Length header
            if !line.starts_with("Content-Length:") {
                continue;
            }
            
            let content_length: usize = line["Content-Length:".len()..]
                .trim()
                .parse()
                .unwrap_or(0);
            
            // Skip the empty line
            let mut buffer = String::new();
            if reader.read_line(&mut buffer).is_err() {
                break;
            }
            
            // Read the message content
            let mut content = vec![0; content_length];
            if reader.read_exact(&mut content).is_err() {
                break;
            }
            
            let content = match String::from_utf8(content) {
                Ok(content) => content,
                Err(_) => continue,
            };
            
            // Parse and handle the message
            if let Ok(request) = serde_json::from_str::<RequestMessage>(&content) {
                let response = self.handle_request(request);
                if let Some(response) = response {
                    let response_json = serde_json::to_string(&response).unwrap();
                    let response_message = format!(
                        "Content-Length: {}\r\n\r\n{}",
                        response_json.len(),
                        response_json
                    );
                    if writer.write_all(response_message.as_bytes()).is_err() {
                        break;
                    }
                }
            } else if let Ok(notification) = serde_json::from_str::<NotificationMessage>(&content) {
                self.handle_notification(notification, &mut writer);
            }
        }
    }
    
    fn handle_request(&self, request: RequestMessage) -> Option<ResponseMessage> {
        match request.method.as_str() {
            "initialize" => {
                Some(ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "capabilities": self.capabilities,
                    })),
                    error: None,
                })
            }
            "textDocument/completion" => {
                // Implement completion logic here
                Some(ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!([
                        {
                            "label": "println",
                            "kind": 3
                        }
                    ])),
                    error: None,
                })
            }
            "textDocument/hover" => {
                // Handle hover request
                let response = ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "contents": {
                            "kind": "markdown",
                            "value": "**SafeLang Documentation**\n\nThis is a hover tooltip for the symbol."
                        }
                    })),
                    error: None,
                };
                
                self.send_response(response, writer)?;
            }
            "textDocument/definition" => {
                // Handle go to definition request
                let response = ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result
}