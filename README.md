# Rust Basics

### Description

This is a simple program that demonstrates the basic concepts of Rust programming language.

### Concepts Demonstrated
1. Variables
2. Data Types
3. Functions
4. Control Flow
5. Error Handling
6. Modules
7. Structs
8. Enums
9. Traits
10. Generics

### Prerequisites
1. Rust Programming Language
2. Cargo Package Manager
3. Git
4. Code Editor (VS Code, IntelliJ IDEA, etc.)
5. Terminal

### Installation
1. Install Rust Programming Language:
```bash
# On MacOS and Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
> For Windows, refer to the official [Rust Installation Guide](https://forge.rust-lang.org/infra/other-installation-methods.html) 


### Running the program
1. Clone the repository:
```bash
git clone https://github.com/TheBlogorithm/learn-rust.git
```

2. In Debug mode:
```bash
cargo run
```

3. In Release mode:
```bash
cargo run --release
```

### Key Points
1. Rust is a statically typed language. It performs type checking at compile time. Tries to figure out the type of the variable based on the value assigned to it. We can also explicitly specify the type.
2. Variables are immutable by default. Use `mut` keyword to make them mutable.
3. Rust has a concept of ownership. Each value in Rust has a variable that’s called its owner. There can only be one owner at a time. When the owner goes out of scope, the value will be dropped.
4. Memory Safety is guaranteed by Rust. It prevents null pointer dereferencing, dangling pointers, buffer overflows, etc.
5. 


### Author
[Arfath Ahmed Syed]()