use crate::ast::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub struct OwnershipChecker {
    symbol_table: HashMap<String, (OwnershipType, LifetimeInfo)>,
}

#[derive(Debug, Clone)]
pub enum OwnershipType {
    Unique,
    Shared,
    Immutable,
}

impl OwnershipChecker {
    pub fn new() -> Self {
        OwnershipChecker {
            symbol_table: HashMap::new(),
        }
    }

    pub fn check(&mut self, ast: &AST) -> Vec<CompileError> {
        let mut errors = Vec::new();
        // Implement ownership rule validation
        match expr {
            Expr::Assignment(left, right) => {
                if !self.is_mutable(left) {
                    errors.push(CompileError::new(
                        "Cannot assign to immutable binding",
                        left.span(),
                    ));
                }
                self.check_borrow_rules(func, args);
            },
            Expr::FunctionCall(func, args) => {
                self.check_borrow_rules(func, args);
            },
        }
        errors
    }

    pub fn check_borrow_rules(&mut self, borrow: &Borrow) -> Vec<CompileError> {
        let mut errors = Vec::new();
        
        match borrow {
            Borrow::Shared(span) => {
                if let Some(owner) = &self.current_owner {
                    if self.ownership_table.get(owner) == Some(&OwnershipType::Unique) {
                        errors.push(CompileError::new(
                            "Cannot create shared borrow of uniquely owned value",
                            *span
                        ));
                    }
                }
            }
            Borrow::Mutable(span) => {
                if self.active_borrows.iter().any(|b| matches!(b, Borrow::Mutable(_))) {
                    errors.push(CompileError::new(
                        "Cannot create mutable borrow while another exists",
                        *span
                    ));
                }
            }

    pub fn validate_thread_safety(&self, expr: &Expr) -> Vec<CompileError> {
        let mut errors = Vec::new();
        match expr {
            Expr::ThreadSpawn(closure, span) => {
                if !self.is_send_safe(closure) {
                    errors.push(CompileError::ThreadSafety(
                        "Closure contains non-Send types".into(),
                        *span
                    ));
                }
            },
            Expr::AtomicAccess(_, span) => {
                if !self.current_scope.is_atomic_context() {
                    errors.push(CompileError::MemorySafety(
                        "Atomic access outside atomic block".into(),
                        *span
                    ));
                }
            }
        }
        errors
    }

    pub fn analyze_lifetimes(&self, ast: &AST) -> Vec<CompileError> {
        let mut errors = Vec::new();
        // Implement lifetime validation rules from documentation
        match expr {
            Expr::ThreadSpawn(closure, span) => {
                if !self.is_send_safe(closure) {
                    errors.push(CompileError::ThreadSafety(
                        "Closure contains non-Send types".into(),
                        *span
                    ));
                }
            },
            Expr::AtomicAccess(_, span) => {
                if !self.current_scope.is_atomic_context() {
                    errors.push(CompileError::MemorySafety(
                        "Atomic access outside atomic block".into(),
                        *span
                    ));
                }
            }
        }
        errors
    }

    fn check_borrow_scope(&self, borrow: &Borrow) -> Result<(), CompileError> {
        // Verify borrow doesn't outlive original value
        match expr {
            Expr::ThreadSpawn(closure, span) => {
                if !self.is_send_safe(closure) {
                    errors.push(CompileError::ThreadSafety(
                        "Closure contains non-Send types".into(),
                        *span
                    ));
                }
            },
            Expr::AtomicAccess(_, span) => {
                if !self.current_scope.is_atomic_context() {
                    errors.push(CompileError::MemorySafety(
                        "Atomic access outside atomic block".into(),
                        *span
                    ));
                }
            }
        }
        errors
    }
}

// Reference counter for safe memory management
pub struct SafeRef<T> {
    inner: Rc<T>,
}

impl<T> SafeRef<T> {
    pub fn new(value: T) -> Self {
        SafeRef {
            inner: Rc::new(value),
        }
    }
    
    pub fn clone(&self) -> Self {
        SafeRef {
            inner: Rc::clone(&self.inner),
        }
    }
    
    pub fn get_ref(&self) -> &T {
        &self.inner
    }
}

// Mutable reference with borrow checking
pub struct SafeMut<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> SafeMut<T> {
    pub fn new(value: T) -> Self {
        SafeMut {
            inner: Rc::new(RefCell::new(value)),
        }
    }
    
    pub fn clone(&self) -> Self {
        SafeMut {
            inner: Rc::clone(&self.inner),
        }
    }
    
    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }
    
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}

// Ownership tracker for compile-time safety checks
pub struct OwnershipTracker {
    variables: HashMap<String, OwnershipState>,
}

enum OwnershipState {
    Owned,
    Moved,
    Borrowed(usize),
    MutBorrowed,
}

impl OwnershipTracker {
    pub fn new() -> Self {
        OwnershipTracker {
            variables: HashMap::new(),
        }
    }
    
    pub fn declare(&mut self, name: &str) -> Result<(), OwnershipError> {
        if self.variables.contains_key(name) {
            return Err(OwnershipError::AlreadyDeclared(name.to_string()));
        }
        
        self.variables.insert(name.to_string(), OwnershipState::Owned);
        Ok(())
    }
    
    pub fn move_ownership(&mut self, from: &str, to: &str) -> Result<(), OwnershipError> {
        // Check if source variable exists and is owned
        match self.variables.get(from) {
            Some(OwnershipState::Owned) => {
                // Mark source as moved
                self.variables.insert(from.to_string(), OwnershipState::Moved);
                
                // Mark destination as owned
                self.variables.insert(to.to_string(), OwnershipState::Owned);
                
                Ok(())
            }
            Some(OwnershipState::Moved) => {
                Err(OwnershipError::UseAfterMove(from.to_string()))
            }
            Some(OwnershipState::Borrowed(_)) => {
                Err(OwnershipError::MoveWhileBorrowed(from.to_string()))
            }
            Some(OwnershipState::MutBorrowed) => {
                Err(OwnershipError::MoveWhileBorrowed(from.to_string()))
            }
            None => {
                Err(OwnershipError::Undeclared(from.to_string()))
            }
        }
    }
    
    pub fn borrow(&mut self, name: &str) -> Result<(), OwnershipError> {
        match self.variables.get(name) {
            Some(OwnershipState::Owned) => {
                // Increment borrow count
                self.variables.insert(name.to_string(), OwnershipState::Borrowed(1));
                Ok(())
            }
            Some(OwnershipState::Borrowed(count)) => {
                // Increment borrow count
                self.variables.insert(name.to_string(), OwnershipState::Borrowed(count + 1));
                Ok(())
            }
            Some(OwnershipState::Moved) => {
                Err(OwnershipError::UseAfterMove(name.to_string()))
            }
            Some(OwnershipState::MutBorrowed) => {
                Err(OwnershipError::BorrowWhileMutBorrowed(name.to_string()))
            }
            None => {
                Err(OwnershipError::Undeclared(name.to_string()))
            }
        }
    }
    
    pub fn borrow_mut(&mut self, name: &str) -> Result<(), OwnershipError> {
        match self.variables.get(name) {
            Some(OwnershipState::Owned) => {
                // Mark as mutably borrowed
                self.variables.insert(name.to_string(), OwnershipState::MutBorrowed);
                Ok(())
            }
            Some(OwnershipState::Borrowed(_)) => {
                Err(OwnershipError::MutBorrowWhileBorrowed(name.to_string()))
            }
            Some(OwnershipState::Moved) => {
                Err(OwnershipError::UseAfterMove(name.to_string()))
            }
            Some(OwnershipState::MutBorrowed) => {
                Err(OwnershipError::MutBorrowWhileMutBorrowed(name.to_string()))
            }
            None => {
                Err(OwnershipError::Undeclared(name.to_string()))
            }
        }
    }
    
    pub fn release_borrow(&mut self, name: &str) -> Result<(), OwnershipError> {
        match self.variables.get(name) {
            Some(OwnershipState::Borrowed(1)) => {
                // Last borrow released, return to owned state
                self.variables.insert(name.to_string(), OwnershipState::Owned);
                Ok(())
            }
            Some(OwnershipState::Borrowed(count)) => {
                // Decrement borrow count
                self.variables.insert(name.to_string(), OwnershipState::Borrowed(count - 1));
                Ok(())
            }
            Some(OwnershipState::MutBorrowed) => {
                // Release mutable borrow, return to owned state
                self.variables.insert(name.to_string(), OwnershipState::Owned);
                Ok(())
            }
            Some(OwnershipState::Owned) => {
                Err(OwnershipError::ReleaseUnborrowed(name.to_string()))
            }
            Some(OwnershipState::Moved) => {
                Err(OwnershipError::UseAfterMove(name.to_string()))
            }
            None => {
                Err(OwnershipError::Undeclared(name.to_string()))
            }
        }
    }
}

#[derive(Debug)]
pub enum OwnershipError {
    AlreadyDeclared(String),
    Undeclared(String),
    UseAfterMove(String),
    MoveWhileBorrowed(String),
    BorrowWhileMutBorrowed(String),
    MutBorrowWhileBorrowed(String),
    MutBorrowWhileMutBorrowed(String),
    ReleaseUnborrowed(String),
}

impl std::fmt::Display for OwnershipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnershipError::AlreadyDeclared(name) => {
                write!(f, "Variable '{}' is already declared", name)
            }
            OwnershipError::Undeclared(name) => {
                write!(f, "Variable '{}' is not declared", name)
            }
            OwnershipError::UseAfterMove(name) => {
                write!(f, "Variable '{}' used after being moved", name)
            }
            OwnershipError::MoveWhileBorrowed(name) => {
                write!(f, "Cannot move variable '{}' while it is borrowed", name)
            }
            OwnershipError::BorrowWhileMutBorrowed(name) => {
                write!(f, "Cannot borrow variable '{}' while it is mutably borrowed", name)
            }
            OwnershipError::MutBorrowWhileBorrowed(name) => {
                write!(f, "Cannot mutably borrow variable '{}' while it is already borrowed", name)
            }
            OwnershipError::MutBorrowWhileMutBorrowed(name) => {
                write!(f, "Cannot mutably borrow variable '{}' while it is already mutably borrowed", name)
            }
            OwnershipError::ReleaseUnborrowed(name) => {
                write!(f, "Cannot release borrow on variable '{}' that is not borrowed", name)
            }
        }
    }
}

impl std::error::Error for OwnershipError {}