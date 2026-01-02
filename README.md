# LR Parser

A Rust library for building **LR(0) parser tables at compile-time** using procedural macros. This project enables you to define context-free grammars using a custom syntax, and automatically generates efficient parsing tables and AST types during compilation.

## Overview

This library provides compile-time evaluation of grammars, creating parser tables and type-safe AST structures before your program runs. By leveraging Rust's procedural macro system, the parser tables are built entirely at compile-time, resulting in:

- **Zero runtime overhead** for table construction
- **Type-safe AST generation** with automatically created enums
- **Clean, declarative grammar syntax**
- **Fast parsing performance** with pre-computed LR(0) tables

## Key Features

- **Compile-time table generation**: LR(0) parser tables are built during compilation, not at runtime
- **Custom grammar syntax**: Define grammars using an intuitive macro-based DSL
- **Automatic AST generation**: Enums for non-terminals are automatically created with variants for each production rule
- **Regex-based terminals**: Support for regex patterns in terminal symbols
- **Type-safe parsing**: Parse results are strongly typed based on your grammar

## Quick Start

```rust
use lr_parser::build_parser;

build_parser! {
    S: A -> "a", B;
    B: C -> "c", B;
    B: D -> "d";
}

// Parse a string
let result = Parser::parse("accd")?;
// Result is a Box<S> where S::A contains the parsed AST
```

## Grammar Syntax

The `build_parser!` macro accepts a grammar defined as a series of rules:

```
<symbol>: <variant_name> -> <elements>;
```

Where:
- **`<symbol>`**: A non-terminal symbol (identifier)
- **`<variant_name>`**: The name for this production rule (becomes an enum variant)
- **`<elements>`**: Comma-separated list of terminals (strings/regex) and non-terminals (identifiers)

### Example: Calculator Grammar

```rust
build_parser! {
    S: Add -> S, "\\+", Term;
    S: Sub -> S, "-", Term;
    S: T -> Term;
    Term: V -> Value;
    Term: Mul -> Term, "\\*", Value;
    Term: Div -> Term, "/", Value;
    Value: Num -> "[0-9]+";
}
```

This grammar:
- Defines a start symbol `S` with three variants: `Add`, `Sub`, and `T`
- Creates a `Term` non-terminal with multiplication and division variants
- Uses regex patterns for numeric values
- Automatically generates enums `S`, `Term`, and `Value` with appropriate variants

## Project Structure

The project consists of three main crates:

- **`lr_parser`**: Main library providing the parser runtime and public API
- **`macros`**: Procedural macro crate that generates parser tables and AST types
- **`common`**: Shared types and traits used across crates

## Requirements

- Rust edition 2021 or later
- The `regex` crate for terminal pattern matching

## License

[Add your license here]
