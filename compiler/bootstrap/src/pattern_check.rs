use crate::ast::*;
use crate::error::CompileError;
use std::collections::HashSet;

pub struct ExhaustivenessChecker {
    type_registry: TypeRegistry,
}

impl ExhaustivenessChecker {
    pub fn new(type_registry: TypeRegistry) -> Self {
        ExhaustivenessChecker { type_registry }
    }
    
    pub fn check(&self, match_expr: &MatchExpr) -> Vec<CompileError> {
        let mut errors = Vec::new();
        
        match &match_expr.scrutinee_type {
            Type::Enum(enum_name) => {
                let enum_def = self.type_registry.get_enum(enum_name);
                if let Some(enum_def) = enum_def {
                    let covered_variants: HashSet<_> = match_expr.arms
                        .iter()
                        .filter_map(|arm| {
                            if let Pattern::EnumVariant(name, _) = &arm.pattern {
                                Some(name.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    let all_variants: HashSet<_> = enum_def.variants
                        .iter()
                        .map(|v| v.name.clone())
                        .collect();
                    
                    let missing: Vec<_> = all_variants.difference(&covered_variants).collect();
                    if !missing.is_empty() && !match_expr.has_wildcard_pattern() {
                        errors.push(CompileError::new(
                            format!("Match is not exhaustive, missing variants: {:?}", missing),
                            match_expr.span,
                        ));
                    }
                }
            },
            // Handle other types (bool, etc.)
            // ... existing code ...
        }
        
        errors
    }
}