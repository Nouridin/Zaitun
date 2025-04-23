# Zaitun Programming Language
Zaitun is a modern, memory-safe programming language designed with strong typing, static analysis, and object-oriented principles. This repository contains the implementation of the Zaitun language, including the compiler, standard library, and developer tools.

## Features
- Memory Safety : Built-in ownership model prevents memory-related bugs
- Strong Type System : Robust static typing with user-defined types
- Object-Oriented : Support for classes, interfaces, and inheritance
- Progressive Disclosure : Simple operations are straightforward, advanced features available when needed
- Batteries Included : Comprehensive standard library
## Project Structure
```plaintext
zaitun/
├── compiler/
│   └── bootstrap/       # Initial compiler implementation
├── std/                 # Standard library
│   └── src/             # Source code for standard library
├── tools/
│   ├── lsp/             # Language Server Protocol implementation
│   ├── repl/            # Interactive REPL
│   └── docs/            # Documentation generator
└── docs/                # Language documentation
 ```
```

## Getting Started
### Prerequisites
- Rust 1.50 or later
- CMake 3.10 or later
- LLVM 12 or later (for native code generation)
### Building from Source
```bash
git clone https://github.com/Nouridin/zaitun.git
cd zaitun
cargo build --release
 ```
```

### Running a Zaitun Program
```bash
zaitun run path/to/program.ztn
 ```

### Creating a New Project
```bash
zaitun init my_project
cd my_project
zaitun build
zaitun run
 ```

## Language Overview
### Hello World
```plaintext
fn main() {
    println("Hello, World!");
}
 ```

### Variables and Types
```plaintext
let name: String = "Zaitun";
let version: f64 = 0.1;
let is_awesome: bool = true;
 ```

### Functions
```plaintext
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
 ```

### Classes and Objects
```plaintext
class Person {
    name: String;
    age: i32;
    
    fn new(name: String, age: i32) -> Person {
        return Person { name: name, age: age };
    }
    
    fn greet(&self) {
        println("Hello, my name is " + self.name);
    }
}
 ```
```

## Development Roadmap
1. Phase 1 : Bootstrap Compiler
   
   - Create a minimal compiler in Rust
   - Implement core language features
2. Phase 2 : Self-Hosting
   
   - Reimplement the compiler in Zaitun itself
   - Add more language features and optimizations
3. Phase 3 : Standard Library
   
   - Implement core types and functions
   - Ensure memory safety and performance
4. Phase 4 : Tools
   
   - Develop package manager
   - Create documentation generator
   - Build language server
## Implementation Priorities
1. Memory Safety : First ensure all safety mechanisms work correctly
2. Correctness : Ensure compiler produces correct code
3. Performance : Optimize for speed and memory usage
4. Developer Experience : Make the language pleasant to use
## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch ( git checkout -b feature/amazing-feature )
3. Commit your changes ( git commit -m 'Add some amazing feature' )
4. Push to the branch ( git push origin feature/amazing-feature )
5. Open a Pull Request
## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- Inspired by modern programming languages like Rust, Swift, and TypeScript
- Built with a focus on developer productivity and code safety
---
*This project is currently in version 0.1.0 draft stage and is under active development.*