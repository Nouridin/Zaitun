use crate::ast::*;
use crate::error::CompileError;

pub struct FFIChecker {
    allowed_unsafe: bool,
}

impl FFIChecker {
    pub fn new(allowed_unsafe: bool) -> Self {
        FFIChecker { allowed_unsafe }
    }
    
    pub fn check_foreign_call(&self, call: &ForeignCall) -> Vec<CompileError> {
        let mut errors = Vec::new();
        
        if !self.allowed_unsafe && !call.is_safe {
            errors.push(CompileError::new(
                "Unsafe foreign call not allowed in this context",
                call.span,
            ));
        }
        
        // Check parameter types for FFI compatibility
        // ... implementation details ...
        
        errors
    }
}