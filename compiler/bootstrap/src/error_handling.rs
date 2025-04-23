use std::fmt;
use std::path::PathBuf;

/// Represents a location in source code
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Represents a span of source code
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.start, self.end)
    }
}

/// Types of errors that can occur during compilation
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Syntax,
    Type,
    Name,
    Reference,
    Ownership,
    Safety,
    IO,
    Internal,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Syntax => write!(f, "Syntax error"),
            ErrorKind::Type => write!(f, "Type error"),
            ErrorKind::Name => write!(f, "Name error"),
            ErrorKind::Reference => write!(f, "Reference error"),
            ErrorKind::Ownership => write!(f, "Ownership error"),
            ErrorKind::Safety => write!(f, "Safety error"),
            ErrorKind::IO => write!(f, "I/O error"),
            ErrorKind::Internal => write!(f, "Internal compiler error"),
        }
    }
}

/// Represents a compiler error
#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: ErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

impl CompileError {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        CompileError {
            kind,
            message: message.to_string(),
            span: None,
            notes: Vec::new(),
            help: None,
        }
    }
    
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
    
    pub fn with_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }
    
    pub fn with_help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        self
    }
    
    /// Format the error with source code context
    pub fn format_with_source(&self, source_code: &str) -> String {
        let mut result = format!("{}: {}\n", self.kind, self.message);
        
        if let Some(span) = &self.span {
            result.push_str(&format!("  --> {}\n", span.start));
            
            // Extract the relevant line from source code
            if let Some(line) = source_code.lines().nth(span.start.line - 1) {
                result.push_str("   |\n");
                result.push_str(&format!("{:4} | {}\n", span.start.line, line));
                result.push_str("   | ");
                
                // Add caret pointing to the error
                for _ in 0..span.start.column - 1 {
                    result.push(' ');
                }
                
                let length = if span.end.line == span.start.line {
                    span.end.column - span.start.column
                } else {
                    line.len() - span.start.column + 1
                };
                
                for _ in 0..length.max(1) {
                    result.push('^');
                }
                
                result.push('\n');
            }
        }
        
        // Add notes
        for note in &self.notes {
            result.push_str(&format!("note: {}\n", note));
        }
        
        // Add help message
        if let Some(help) = &self.help {
            result.push_str(&format!("help: {}\n", help));
        }
        
        result
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)?;
        
        if let Some(span) = &self.span {
            write!(f, " at {}", span.start)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for CompileError {}

/// Collection of errors and warnings
#[derive(Debug, Default)]
pub struct Diagnostics {
    errors: Vec<CompileError>,
    warnings: Vec<CompileError>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: CompileError) {
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: CompileError) {
        self.warnings.push(warning);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
    
    pub fn format_all(&self, source_map: &SourceMap) -> String {
        let mut result = String::new();
        
        for error in &self.errors {
            if let Some(span) = &error.span {
                if let Some(source) = source_map.get_source(&span.start.file) {
                    result.push_str(&error.format_with_source(source));
                } else {
                    result.push_str(&format!("{}\n", error));
                }
            } else {
                result.push_str(&format!("{}\n", error));
            }
            result.push('\n');
        }
        
        for warning in &self.warnings {
            if let Some(span) = &warning.span {
                if let Some(source) = source_map.get_source(&span.start.file) {
                    result.push_str(&warning.format_with_source(source));
                } else {
                    result.push_str(&format!("{}\n", warning));
                }
            } else {
                result.push_str(&format!("{}\n", warning));
            }
            result.push('\n');
        }
        
        result
    }
}

/// Manages source code files
#[derive(Debug, Default)]
pub struct SourceMap {
    sources: std::collections::HashMap<PathBuf, String>,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            sources: std::collections::HashMap::new(),
        }
    }
    
    pub fn add_source(&mut self, path: PathBuf, source: String) {
        self.sources.insert(path, source);
    }
    
    pub fn get_source(&self, path: &PathBuf) -> Option<&str> {
        self.sources.get(path).map(|s| s.as_str())
    }
}