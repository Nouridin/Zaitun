# Zaitun Language Specification

## Lexical Structure
identifier → [A-Za-z_][A-Za-z0-9_]*
literal → integer | float | string | boolean
integer → [0-9]+
float → [0-9]+.[0-9]+
string → "[^"]*"
boolean → true | false

## Basic Syntax
program → (function_decl | struct_decl)*
function_decl → "fn" identifier "(" params? ")" "->" type block
struct_decl → "struct" identifier "{" (field ",")* "}"