use crate::ast::*;
use crate::error::CompileError;

pub struct TypeChecker {
    symbol_table: HashMap<String, TypeInfo>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: HashMap::new(),
        }
    }

    pub fn check(&mut self, ast: &AST) -> Vec<CompileError> {
        let mut errors = Vec::new();
        // Implement type rule validation
        // ... existing code ...
        errors
    }

    fn check_interface_impl(&self, impl_block: &InterfaceImpl) -> Vec<CompileError> {
        let mut errors = Vec::new();
        // Verify all interface requirements are met
        // ... existing code ...
        errors
    }
}

    fn check_binary_op(&self, op: &BinOp, left: &Type, right: &Type) -> Result<Type, CompileError> {
        match (left, right) {
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Float, Type::Float) => Ok(Type::Float),
            // ... existing code ...
            _ => Err(CompileError::TypeMismatch {
                expected: format!("{}", left),
                found: format!("{}", right),
                span: op.span,
            }),
        }
    }
}