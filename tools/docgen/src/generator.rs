use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct DocGenerator {
    source_dirs: Vec<PathBuf>,
    output_dir: PathBuf,
    templates: HashMap<String, String>,
    config: DocConfig,
}

impl DocGenerator {
    pub fn new(output_dir: &Path) -> Self {
        DocGenerator {
            source_dirs: Vec::new(),
            output_dir: output_dir.to_path_buf(),
            templates: HashMap::new(),
            config: DocConfig::default(),
        }
    }
    
    pub fn add_source_dir(&mut self, dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            self.source_dirs.push(dir.to_path_buf());
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source directory not found: {}", dir.display()),
            ))
        }
    }
    
    pub fn load_template(&mut self, name: &str, path: &Path) -> io::Result<()> {
        let template = fs::read_to_string(path)?;
        self.templates.insert(name.to_string(), template);
        Ok(())
    }
    
    pub fn set_config(&mut self, config: DocConfig) {
        self.config = config;
    }
    
    pub fn generate(&self) -> io::Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)?;
        
        // Parse source files and extract documentation
        let mut docs = Vec::new();
        for dir in &self.source_dirs {
            self.process_directory(dir, &mut docs)?;
        }
        
        // Generate index page
        self.generate_index(&docs)?;
        
        // Generate individual pages
        for doc in &docs {
            self.generate_page(doc)?;
        }
        
        // Generate search index
        self.generate_search_index(&docs)?;
        
        // Copy static assets
        self.copy_assets()?;
        
        Ok(())
    }
    
    fn process_directory(&self, dir: &Path, docs: &mut Vec<DocItem>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.process_directory(&path, docs)?;
            } else if let Some(ext) = path.extension() {
                if ext == "safe" {
                    let doc_items = self.parse_file(&path)?;
                    docs.extend(doc_items);
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_file(&self, file: &Path) -> io::Result<Vec<DocItem>> {
        let content = fs::read_to_string(file)?;
        let mut items = Vec::new();
        
        // Parse documentation comments
        let mut in_doc_comment = false;
        let mut current_doc = String::new();
        let mut line_number = 0;
        
        for line in content.lines() {
            line_number += 1;
            
            if line.trim().starts_with("///") {
                // Single-line doc comment
                let doc_text = line.trim_start_matches("///").trim();
                if !in_doc_comment {
                    in_doc_comment = true;
                    current_doc.clear();
                }
                current_doc.push_str(doc_text);
                current_doc.push('\n');
            } else if in_doc_comment {
                // End of doc comment block, parse the following declaration
                in_doc_comment = false;
                
                if let Some(item) = self.parse_declaration(line, &current_doc, file, line_number) {
                    items.push(item);
                }
                
                current_doc.clear();
            }
        }
        
        Ok(items)
    }
    
    fn parse_declaration(&self, line: &str, doc_text: &str, file: &Path, line_number: usize) -> Option<DocItem> {
        let line = line.trim();
        
        // Check for different declaration types
        if line.starts_with("class ") {
            let name = line.strip_prefix("class ")?.split_whitespace().next()?;
            Some(DocItem {
                name: name.to_string(),
                kind: DocItemKind::Class,
                documentation: doc_text.to_string(),
                source_file: file.to_path_buf(),
                line_number,
                members: Vec::new(),
            })
        } else if line.starts_with("function ") {
            let name = line.strip_prefix("function ")?.split('(').next()?.trim();
            Some(DocItem {
                name: name.to_string(),
                kind: DocItemKind::Function,
                documentation: doc_text.to_string(),
                source_file: file.to_path_buf(),
                line_number,
                members: Vec::new(),
            })
        } else if line.starts_with("struct ") {
            let name = line.strip_prefix("struct ")?.split_whitespace().next()?;
            Some(DocItem {
                name: name.to_string(),
                kind: DocItemKind::Struct,
                documentation: doc_text.to_string(),
                source_file: file.to_path_buf(),
                line_number,
                members: Vec::new(),
            })
        } else if line.starts_with("enum ") {
            let name = line.strip_prefix("enum ")?.split_whitespace().next()?;
            Some(DocItem {
                name: name.to_string(),
                kind: DocItemKind::Enum,
                documentation: doc_text.to_string(),
                source_file: file.to_path_buf(),
                line_number,
                members: Vec::new(),
            })
        } else if line.starts_with("interface ") {
            let name = line.strip_prefix("interface ")?.split_whitespace().next()?;
            Some(DocItem {
                name: name.to_string(),
                kind: DocItemKind::Interface,
                documentation: doc_text.to_string(),
                source_file: file.to_path_buf(),
                line_number,
                members: Vec::new(),
            })
        } else {
            None
        }
    }
    
    fn generate_index(&self, docs: &[DocItem]) -> io::Result<()> {
        let index_path = self.output_dir.join("index.html");
        let mut file = File::create(index_path)?;
        
        let template = self.templates.get("index").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Index template not found")
        })?;
        
        // Group items by kind
        let mut classes = Vec::new();
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut interfaces = Vec::new();
        
        for item in docs {
            match item.kind {
                DocItemKind::Class => classes.push(item),
                DocItemKind::Function => functions.push(item),
                DocItemKind::Struct => structs.push(item),
                DocItemKind::Enum => enums.push(item),
                DocItemKind::Interface => interfaces.push(item),
            }
        }
        
        // Generate HTML content
        let mut content = template.clone();
        content = content.replace("{{title}}", &self.config.title);
        content = content.replace("{{version}}", &self.config.version);
        
        // Replace sections with actual content
        let classes_html = self.generate_item_list(&classes, "Classes");
        let functions_html = self.generate_item_list(&functions, "Functions");
        let structs_html = self.generate_item_list(&structs, "Structs");
        let enums_html = self.generate_item_list(&enums, "Enums");
        let interfaces_html = self.generate_item_list(&interfaces, "Interfaces");
        
        content = content.replace("{{classes}}", &classes_html);
        content = content.replace("{{functions}}", &functions_html);
        content = content.replace("{{structs}}", &structs_html);
        content = content.replace("{{enums}}", &enums_html);
        content = content.replace("{{interfaces}}", &interfaces_html);
        
        file.write_all(content.as_bytes())?;
        
        Ok(())
    }
    
    fn generate_item_list(&self, items: &[&DocItem], title: &str) -> String {
        if items.is_empty() {
            return String::new();
        }
        
        let mut html = format!("<h2>{}</h2>\n<ul>\n", title);
        
        for item in items {
            let item_url = format!("{}.html", item.name);
            html.push_str(&format!(
                "  <li><a href=\"{}\">{}</a> - {}</li>\n",
                item_url,
                item.name,
                item.documentation.lines().next().unwrap_or("").trim()
            ));
        }
        
        html.push_str("</ul>\n");
        html
    }
    
    fn generate_page(&self, doc: &DocItem) -> io::Result<()> {
        let page_path = self.output_dir.join(format!("{}.html", doc.name));
        let mut file = File::create(page_path)?;
        
        let template = self.templates.get("item").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Item template not found")
        })?;
        
        // Generate HTML content
        let mut content = template.clone();
        content = content.replace("{{title}}", &format!("{} - {}", doc.name, self.config.title));
        content = content.replace("{{version}}", &self.config.version);
        content = content.replace("{{name}}", &doc.name);
        content = content.replace("{{kind}}", &format!("{:?}", doc.kind));
        
        // Convert documentation to HTML
        let doc_html = self.markdown_to_html(&doc.documentation);
        content = content.replace("{{documentation}}", &doc_html);
        
        // Source file information
        let source_info = format!(
            "Defined in <code>{}</code> at line {}",
            doc.source_file.display(),
            doc.line_number
        );
        content = content.replace("{{source}}", &source_info);
        
        // Members list
        let members_html = if doc.members.is_empty() {
            String::new()
        } else {
            let mut html = String::from("<h2>Members</h2>\n<ul>\n");
            for member in &doc.members {
                html.push_str(&format!(
                    "  <li><code>{}</code> - {}</li>\n",
                    member.name,
                    member.documentation.lines().next().unwrap_or("").trim()
                ));
            }
            html.push_str("</ul>\n");
            html
        };
        content = content.replace("{{members}}", &members_html);
        
        file.write_all(content.as_bytes())?;
        
        Ok(())
    }
    
    fn generate_search_index(&self, docs: &[DocItem]) -> io::Result<()> {
        let index_path = self.output_dir.join("search-index.js");
        let mut file = File::create(index_path)?;
        
        // Create search index data
        let mut index_data = String::from("window.searchIndex = [\n");
        
        for (i, doc) in docs.iter().enumerate() {
            let summary = doc.documentation.lines().next().unwrap_or("").trim();
            index_data.push_str(&format!(
                "  {{\"name\":\"{}\",\"kind\":\"{:?}\",\"summary\":\"{}\",\"url\":\"{}.html\"}}",
                doc.name,
                doc.kind,
                summary,
                doc.name
            ));
            
            if i < docs.len() - 1 {
                index_data.push_str(",\n");
            } else {
                index_data.push_str("\n");
            }
        }
        
        index_data.push_str("];\n");
        file.write_all(index_data.as_bytes())?;
        
        Ok(())
    }
    
    fn copy_assets(&self) -> io::Result<()> {
        let assets_dir = self.output_dir.join("assets");
        fs::create_dir_all(&assets_dir)?;
        
        // Copy CSS
        let css_content = self.templates.get("style").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Style template not found")
        })?;
        
        let css_path = assets_dir.join("style.css");
        let mut css_file = File::create(css_path)?;
        css_file.write_all(css_content.as_bytes())?;
        
        // Copy JavaScript
        let js_content = self.templates.get("script").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Script template not found")
        })?;
        
        let js_path = assets_dir.join("script.js");
        let mut js_file = File::create(js_path)?;
        js_file.write_all(js_content.as_bytes())?;
        
        Ok(())
    }
    
    fn markdown_to_html(&self, markdown: &str) -> String {
        // Simple markdown to HTML conversion
        // In a real implementation, use a proper markdown parser
        let mut html = String::new();
        let mut in_code_block = false;
        let mut in_list = false;
        
        for line in markdown.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("```") {
                if in_code_block {
                    html.push_str("</code></pre>\n");
                } else {
                    html.push_str("<pre><code>");
                }
                in_code_block = !in_code_block;
            } else if in_code_block {
                html.push_str(line);
                html.push('\n');
            } else if trimmed.starts_with("# ") {
                html.push_str(&format!("<h1>{}</h1>\n", &trimmed[2..]));
            } else if trimmed.starts_with("## ") {
                html.push_str(&format!("<h2>{}</h2>\n", &trimmed[3..]));
            } else if trimmed.starts_with("### ") {
                html.push_str(&format!("<h3>{}</h3>\n", &trimmed[4..]));
            } else if trimmed.starts_with("- ") {
                if !in_list {
                    html.push_str("<ul>\n");
                    in_list = true;
                }
                html.push_str(&format!("  <li>{}</li>\n", &trimmed[2..]));
            } else if trimmed.is_empty() && in_list {
                html.push_str("</ul>\n");
                in_list = false;
                html.push_str("<p></p>\n");
            } else if trimmed.is_empty() {
                html.push_str("<p></p>\n");
            } else {
                html.push_str(&format!("<p>{}</p>\n", trimmed));
            }
        }
        
        if in_list {
            html.push_str("</ul>\n");
        }
        
        if in_code_block {
            html.push_str("</code></pre>\n");
        }
        
        html
    }
}

#[derive(Debug, Clone)]
pub struct DocConfig {
    pub title: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub repository: String,
}

impl Default for DocConfig {
    fn default() -> Self {
        DocConfig {
            title: String::from("SafeLang Documentation"),
            version: String::from("0.1.0"),
            author: String::from("SafeLang Team"),
            description: String::from("Documentation for the SafeLang programming language"),
            repository: String::from("https://github.com/safelang/safelang"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocItem {
    pub name: String,
    pub kind: DocItemKind,
    pub documentation: String,
    pub source_file: PathBuf,
    pub line_number: usize,
    pub members: Vec<DocMember>,
}

#[derive(Debug, Clone)]
pub struct DocMember {
    pub name: String,
    pub documentation: String,
}

#[derive(Debug, Clone, Copy)]
pub enum DocItemKind {
    Class,
    Function,
    Struct,
    Enum,
    Interface,
}