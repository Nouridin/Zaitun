use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Bool,
    Int,
    Float,
    String,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Class(String),
    Interface(String),
    Struct(String),
    Enum(String),
    Optional(Box<Type>),
    Union(Vec<Type>),
    Generic(String, Vec<Type>),
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Array(elem_type) => write!(f, "{}[]", elem_type),
            Type::Map(key_type, value_type) => write!(f, "Map<{}, {}>", key_type, value_type),
            Type::Function(param_types, return_type) => {
                write!(f, "function(")?;
                for (i, param_type) in param_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param_type)?;
                }
                write!(f, "): {}", return_type)
            }
            Type::Class(name) => write!(f, "{}", name),
            Type::Interface(name) => write!(f, "{}", name),
            Type::Struct(name) => write!(f, "{}", name),
            Type::Enum(name) => write!(f, "{}", name),
            Type::Optional(inner) => write!(f, "{}?", inner),
            Type::Union(types) => {
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
            Type::Generic(name, params) => {
                write!(f, "{}<", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ">")
            }
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

pub struct TypeChecker {
    type_env: HashMap<String, Type>,
    class_hierarchy: HashMap<String, Vec<String>>,
    interface_implementations: HashMap<String, Vec<String>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            type_env: HashMap::new(),
            class_hierarchy: HashMap::new(),
            interface_implementations: HashMap::new(),
        }
    }
    
    pub fn add_variable(&mut self, name: &str, type_: Type) {
        self.type_env.insert(name.to_string(), type_);
    }
    
    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.type_env.get(name)
    }
    
    pub fn add_class(&mut self, name: &str, parent: Option<&str>) {
        if let Some(parent_name) = parent {
            let entry = self.class_hierarchy.entry(parent_name.to_string()).or_insert_with(Vec::new);
            entry.push(name.to_string());
        }
    }
    
    pub fn add_interface_implementation(&mut self, class_name: &str, interface_name: &str) {
        let entry = self.interface_implementations.entry(interface_name.to_string()).or_insert_with(Vec::new);
        entry.push(class_name.to_string());
    }
    
    pub fn is_subtype(&self, sub: &Type, super_: &Type) -> bool {
        if sub == super_ {
            return true;
        }
        
        match (sub, super_) {
            // Optional type is a supertype of its inner type
            (inner, Type::Optional(super_inner)) => {
                self.is_subtype(inner, super_inner)
            }
            
            // Class hierarchy
            (Type::Class(sub_name), Type::Class(super_name)) => {
                self.is_subclass(sub_name, super_name)
            }
            
            // Interface implementation
            (Type::Class(class_name), Type::Interface(interface_name)) => {
                self.implements_interface(class_name, interface_name)
            }
            
            // Array subtyping is covariant
            (Type::Array(sub_elem), Type::Array(super_elem)) => {
                self.is_subtype(sub_elem, super_elem)
            }
            
            // Function subtyping is contravariant in parameters and covariant in return type
            (Type::Function(sub_params, sub_return), Type::Function(super_params, super_return)) => {
                if sub_params.len() != super_params.len() {
                    return false;
                }
                
                // Parameters are contravariant
                for (super_param, sub_param) in super_params.iter().zip(sub_params.iter()) {
                    if !self.is_subtype(super_param, sub_param) {
                        return false;
                    }
                }
                
                // Return type is covariant
                self.is_subtype(sub_return, super_return)
            }
            
            // Union type is a supertype of all its member types
            (sub_type, Type::Union(union_types)) => {
                union_types.iter().any(|union_type| self.is_subtype(sub_type, union_type))
            }
            
            // A type is a subtype of a union if it's a subtype of any member of the union
            (Type::Union(union_types), super_type) => {
                union_types.iter().all(|union_type| self.is_subtype(union_type, super_type))
            }
            
            _ => false,
        }
    }
    
    fn is_subclass(&self, sub: &str, super_: &str) -> bool {
        if sub == super_ {
            return true;
        }
        
        // Check direct subclasses
        if let Some(subclasses) = self.class_hierarchy.get(super_) {
            if subclasses.contains(&sub.to_string()) {
                return true;
            }
            
            // Check indirect subclasses (recursive)
            for subclass in subclasses {
                if self.is_subclass(sub, subclass) {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn implements_interface(&self, class_name: &str, interface_name: &str) -> bool {
        if let Some(implementers) = self.interface_implementations.get(interface_name) {
            if implementers.contains(&class_name.to_string()) {
                return true;
            }
            
            // Check if any superclass implements the interface
            for implementer in implementers {
                if self.is_subclass(class_name, implementer) {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn check_assignment(&self, target_type: &Type, value_type: &Type) -> Result<(), TypeError> {
        if self.is_subtype(value_type, target_type) {
            Ok(())
        } else {
            Err(TypeError::IncompatibleTypes(
                value_type.to_string(),
                target_type.to_string(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum TypeError {
    UndefinedVariable(String),
    UndefinedType(String),
    IncompatibleTypes(String, String),
    NotCallable(String),
    WrongNumberOfArguments(usize, usize),
    MemberNotFound(String, String),
    NotIndexable(String),
    InvalidOperator(String, String, String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            TypeError::UndefinedType(name) => {
                write!(f, "Undefined type: {}", name)
            }
            TypeError::IncompatibleTypes(from, to) => {
                write!(f, "Cannot assign value of type {} to variable of type {}", from, to)
            }
            TypeError::NotCallable(type_) => {
                write!(f, "Type {} is not callable", type_)
            }
            TypeError::WrongNumberOfArguments(expected, actual) => {
                write!(f, "Expected {} arguments but got {}", expected, actual)
            }
            TypeError::MemberNotFound(type_, member) => {
                write!(f, "Member {} not found in type {}", member, type_)
            }
            TypeError::NotIndexable(type_) => {
                write!(f, "Type {} is not indexable", type_)
            }
            TypeError::InvalidOperator(op, left, right) => {
                write!(f, "Operator {} not defined for types {} and {}", op, left, right)
            }
        }
    }
}

impl std::error::Error for TypeError {}