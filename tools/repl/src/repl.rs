use std::io::{self, Write};
use std::collections::HashMap;

pub struct REPL {
    variables: HashMap<String, Value>,
    history: Vec<String>,
    compiler: Compiler,
    interpreter: Interpreter,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            variables: HashMap::new(),
            history: Vec::new(),
            compiler: Compiler::new(),
            interpreter: Interpreter::new(),
        }
    }
    
    pub fn run(&mut self) -> io::Result<()> {
        println!("SafeLang REPL v0.1.0");
        println!("Type 'help' for available commands, 'exit' to quit");
        
        loop {
            print!("> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            self.history.push(input.to_string());
            
            match input {
                "exit" | "quit" => break,
                "help" => self.print_help(),
                "history" => self.print_history(),
                "clear" => self.clear_variables(),
                _ => self.evaluate(input),
            }
        }
        
        Ok(())
    }
    
    fn evaluate(&mut self, input: &str) {
        // Special case for variable assignment
        if let Some(pos) = input.find('=') {
            if let Some(var_name) = input[..pos].trim().strip_prefix("let ") {
                let var_name = var_name.trim();
                let expr = input[pos + 1..].trim();
                
                match self.eval_expression(expr) {
                    Ok(value) => {
                        self.variables.insert(var_name.to_string(), value.clone());
                        println!("{} = {}", var_name, value);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
                
                return;
            }
        }
        
        // Regular expression evaluation
        match self.eval_expression(input) {
            Ok(value) => {
                println!("{}", value);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    
    fn eval_expression(&self, expr: &str) -> Result<Value, String> {
        // Try to compile the expression
        let ast = self.compiler.parse(expr)
            .map_err(|e| format!("Parse error: {}", e))?;
        
        // Evaluate the AST
        self.interpreter.eval(&ast, &self.variables)
    }
    
    fn print_help(&self) {
        println!("Available commands:");
        println!("  help                 - Show this help message");
        println!("  exit, quit           - Exit the REPL");
        println!("  history              - Show command history");
        println!("  clear                - Clear all variables");
        println!("  let <name> = <expr>  - Assign expression result to variable");
        println!("  <expr>               - Evaluate expression and print result");
    }
    
    fn print_history(&self) {
        for (i, cmd) in self.history.iter().enumerate() {
            println!("{}: {}", i + 1, cmd);
        }
    }
    
    fn clear_variables(&mut self) {
        self.variables.clear();
        println!("All variables cleared");
    }
}

struct Compiler {
    // Compiler implementation
}

impl Compiler {
    fn new() -> Self {
        Compiler {}
    }
    
    fn parse(&self, input: &str) -> Result<AST, String> {
        // Parse input into AST
        // ... implementation details ...
        Ok(AST {})
    }
}

struct Interpreter {
    // Interpreter implementation
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {}
    }
    
    fn eval(&self, ast: &AST, variables: &HashMap<String, Value>) -> Result<Value, String> {
        // Evaluate AST
        // ... implementation details ...
        Ok(Value::Number(0.0))
    }
}

struct AST {
    // AST structure
}

#[derive(Clone, Debug)]
enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Object(fields) => {
                write!(f, "{{")?;
                for (i, (key, value)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
            Value::Null => write!(f, "null"),
        }
    }
}