use crate::ast::*;
use crate::error::CompileError;
use std::collections::HashMap;

pub struct MacroSystem {
    macros: HashMap<String, MacroDefinition>,
}

impl MacroSystem {
    pub fn new() -> Self {
        MacroSystem {
            macros: HashMap::new(),
        }
    }
    
    pub fn register_macro(&mut self, name: String, definition: MacroDefinition) {
        self.macros.insert(name, definition);
    }
    
    pub fn expand_macros(&self, ast: &mut AST) -> Result<(), Vec<CompileError>> {
        let mut errors = Vec::new();
        
        // First pass: collect all macro definitions
        for node in &ast.nodes {
            if let ASTNode::MacroDefinition(def) = node {
                // Register macro definitions
                // ... existing code ...
            }
        }
        
        // Second pass: expand macro invocations
        self.expand_nodes(&mut ast.nodes, &mut errors);
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn expand_nodes(&self, nodes: &mut Vec<ASTNode>, errors: &mut Vec<CompileError>) {
        let mut i = 0;
        while i < nodes.len() {
            if let ASTNode::MacroInvocation(invocation) = &nodes[i] {
                if let Some(macro_def) = self.macros.get(&invocation.name) {
                    match self.expand_invocation(invocation, macro_def) {
                        Ok(expanded) => {
                            // Replace the invocation with expanded nodes
                            nodes.remove(i);
                            for (j, node) in expanded.into_iter().enumerate() {
                                nodes.insert(i + j, node);
                            }
                            i += expanded.len();
                        }
                        Err(err) => {
                            errors.push(err);
                            i += 1;
                        }
                    }
                } else {
                    errors.push(CompileError::new(
                        format!("Undefined macro: {}", invocation.name),
                        invocation.span,
                    ));
                    i += 1;
                }
            } else {
                // Recursively expand macros in child nodes
                // ... existing code ...
                i += 1;
            }
        }
    }
    
    fn expand_invocation(&self, invocation: &MacroInvocation, definition: &MacroDefinition) -> Result<Vec<ASTNode>, CompileError> {
        // Implement macro expansion logic
        // ... existing code ...
        Ok(Vec::new())
    }
}

pub struct MacroDefinition {
    params: Vec<String>,
    body: Vec<ASTNode>,
}

pub struct MacroInvocation {
    name: String,
    args: Vec<ASTNode>,
    span: Span,
}