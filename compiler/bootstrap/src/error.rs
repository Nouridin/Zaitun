use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Span {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: ErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
}

impl CompileError {
    pub fn new(kind: ErrorKind, message: &str, span: Option<Span>) -> Self {
        CompileError {
            kind,
            message: message.to_string(),
            span,
            notes: Vec::new(),
        }
    }
    
    pub fn with_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }
    
    pub fn format_with_source(&self, source_code: &str) -> String {
        let mut result = format!("Error: {}\n", self.message);
        
        if let Some(span) = &self.span {
            result.push_str(&format!("  --> {}\n", span));
            
            // Extract the relevant line from source code
            if let Some(line) = source_code.lines().nth(span.start.line - 1) {
                result.push_str(&format!("   |\n"));
                result.push_str(&format!("{:3} | {}\n", span.start.line, line));
                result.push_str(&format!("   | "));
                
                // Add caret pointing to the error
                for _ in 0..span.start.column - 1 {
                    result.push(' ');
                }
                
                let length = if span.end.line == span.start.line {
                    span.end.column - span.start.column
                } else {
                    line.len() - span.start.column + 1
                };
                
                for _ in 0..length {
                    result.push('^');
                }
                
                result.push('\n');
            }
        }
        
        // Add notes
        for note in &self.notes {
            result.push_str(&format!("Note: {}\n", note));
        }
        
        result
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.message)?;
        
        if let Some(span) = &self.span {
            write!(f, " at {}", span)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for CompileError {}
use thiserror::Error;
use crate::span::Span;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Memory safety violation: {0}")]
    MemorySafety(String, Span),
    
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
        span: Span,
    },
    
    #[error("Thread safety violation: {0}")]
    ThreadSafety(String, Span),

    #[error("Join error: {0}")]
    JoinError(String, Span),
}