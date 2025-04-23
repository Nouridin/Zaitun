use crate::ast::*;
use std::fs;
use std::path::Path;

pub enum DocFormat {
    HTML,
    Markdown,
    PlainText,
}

pub struct DocGenerator {
    ast: AST,
    output_format: DocFormat,
    output_dir: String,
}

impl DocGenerator {
    pub fn new(ast: AST, format: DocFormat, output_dir: &str) -> Self {
        DocGenerator {
            ast,
            output_format: format,
            output_dir: output_dir.to_string(),
        }
    }
    
    pub fn generate(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.output_dir)?;
        
        // Generate documentation for each module
        for node in &self.ast.nodes {
            if let ASTNode::Module(module) = node {
                self.generate_module_doc(module)?;
            }
        }
        
        // Generate index page
        self.generate_index()?;
        
        Ok(())
    }
    
    fn generate_module_doc(&self, module: &Module) -> Result<(), std::io::Error> {
        let path = Path::new(&self.output_dir).join(format!("{}.md", module.name));
        let mut content = String::new();
        
        content.push_str(&format!("# Module {}\n\n", module.name));
        
        // Extract documentation comments
        if let Some(doc) = &module.doc_comment {
            content.push_str(&format!("{}\n\n", doc));
        }
        
        // Document functions
        content.push_str("## Functions\n\n");
        for node in &module.body {
            if let ASTNode::FunctionDecl(func) = node {
                self.document_function(&mut content, func);
            }
        }
        
        // Document types
        content.push_str("## Types\n\n");
        for node in &module.body {
            match node {
                ASTNode::StructDecl(struct_decl) => self.document_struct(&mut content, struct_decl),
                ASTNode::EnumDecl(enum_decl) => self.document_enum(&mut content, enum_decl),
                ASTNode::InterfaceDecl(interface) => self.document_interface(&mut content, interface),
                _ => {}
            }
        }
        
        fs::write(path, content)?;
        Ok(())
    }
    
    fn document_function(&self, content: &mut String, func: &FunctionDecl) {
        content.push_str(&format!("### `{}`\n\n", func.name));
        
        if let Some(doc) = &func.doc_comment {
            content.push_str(&format!("{}\n\n", doc));
        }
        
        content.push_str("**Signature:**\n\n");
        content.push_str("```\n");
        // Format function signature
        content.push_str(&format!("function {}(", func.name));
        
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 {
                content.push_str(", ");
            }
            content.push_str(&format!("{}: {}", param.name, param.type_name));
        }
        
        content.push_str(&format!("): {}\n", func.return_type));
        content.push_str("```\n\n");
    }
    
    fn generate_index(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.output_dir).join("index.md");
        let mut content = String::new();
        
        content.push_str("# SafeLang API Documentation\n\n");
        content.push_str("## Modules\n\n");
        
        for node in &self.ast.nodes {
            if let ASTNode::Module(module) = node {
                content.push_str(&format!("- [{0}]({0}.md)\n", module.name));
            }
        }
        
        fs::write(path, content)?;
        Ok(())
    }
    
    // Additional documentation methods
}