# Detailed Project Instructions

This document provides comprehensive low-level information about the LR Parser project structure, implementation details, and file organization. This is intended for AI agents or developers who need to understand the internal workings of the project.

## Project Overview

This project implements an LR(0) parser generator that builds parsing tables at compile-time using Rust procedural macros. The parser uses a custom grammar syntax and generates type-safe AST structures automatically.

## Architecture

The project is organized into three main crates:

1. **`lr_parser`** (main library): Runtime parser implementation
2. **`macros`** (procedural macro): Compile-time table generation
3. **`common`**: Shared types and traits

## Crate: `lr_parser` (Main Library)

### Location: `/src/`

#### `lib.rs`
- **Purpose**: Main entry point, exports public API
- **Key Exports**:
  - `build_parser!` macro (re-exported from macros crate)
  - `ParserTrait`: Trait for parser instances
  - `Error`: Unified error type (parsing, lexing, alphabet errors)
- **Features**:
  - Conditional lexer module based on `manual_lexing` feature flag
  - Re-exports common types

#### `parser/mod.rs`
- **Purpose**: Core parsing logic implementation
- **Key Components**:
  - `ParseInstance<T: TableTrait>`: Main parsing state machine
  - `parse()`: Entry point that runs the LR parsing algorithm
  - `shift()`: Handles shift actions (consuming tokens)
  - `reduce()`: Handles reduce actions (building AST nodes)
- **Algorithm**: Implements standard LR(0) shift-reduce parsing with a state stack
- **State Management**: Uses `StateMachine` to track parser state transitions
- **Result Stack**: Maintains a stack of `Box<dyn Any>` values that get downcast to specific AST types during reduction

#### `parser/state_machine.rs`
- **Purpose**: Manages parser state transitions
- **Functionality**: Tracks state stack, handles state advances and reverts during reductions

#### `parser/error.rs`
- **Purpose**: Parsing error definitions
- **Error Types**: Expected token errors with position information

#### `lexer/mod.rs`
- **Purpose**: Tokenization/lexical analysis
- **Key Features**:
  - Pattern-based lexing using regex
  - Longest-match strategy for token recognition
  - `Lexer::from_alphabet()`: Creates lexer from terminal patterns
- **Visibility**: Public when `manual_lexing` feature is enabled, otherwise internal

#### `lexer/pattern.rs`
- **Purpose**: Regex pattern wrapper for terminals
- **Functionality**: Converts string patterns to regex for matching

#### `tokens/mod.rs`
- **Purpose**: Token representation
- **Token Types**:
  - `Token::Value { label, value }`: Matched terminal with its value
  - `Token::EOF`: End-of-file marker

## Crate: `macros` (Procedural Macro)

### Location: `/macros/src/`

#### `lib.rs`
- **Purpose**: Entry point for the `build_parser!` procedural macro
- **Key Function**: `build_parser(input: TokenStream) -> TokenStream`
- **Process**:
  1. Parses input as `Grammar`
  2. Generates enum types via `enums()`
  3. Generates parser table via `table::table()`
  4. Generates `Parser` struct via `parser::parser_struct_tokens()`
- **Output**: Complete code generation including types, tables, and parser struct

#### `grammar/mod.rs`
- **Purpose**: Grammar parsing and representation
- **Key Types**:
  - `Grammar`: Represents the parsed grammar structure
  - `Grammar::rules`: `HashMap<NonTerminal, Vec<Variant>>`
- **Parsing**: Uses `syn` crate to parse custom grammar syntax
- **Validation**: Ensures start symbol 'S' exists
- **Grammar Syntax**: Rules separated by semicolons, each rule has format:
  ```
  <symbol>: <name> -> <elements>;
  ```

#### `grammar/rule/mod.rs` and `grammar/rule/variant_parser.rs`
- **Purpose**: Parse individual grammar rules
- **Functionality**: Converts macro input tokens into `Variant` structures

#### `grammar/id_parse.rs`
- **Purpose**: Parse grammar elements (terminals vs non-terminals)
- **Distinction**: String literals become terminals, identifiers become non-terminals

#### `enums/mod.rs`
- **Purpose**: Generate Rust enum types for non-terminals
- **Process**:
  - For each non-terminal symbol, creates an enum
  - Each production rule becomes a variant
  - Variant fields are typed based on rule elements:
    - Terminals → `Box<String>`
    - Non-terminals → `Box<NonTerminalType>`
- **Example Output**:
  ```rust
  pub enum S {
      A(Box<String>, Box<B>),
      B(Box<String>),
  }
  ```

#### `table/mod.rs`
- **Purpose**: Orchestrates table generation
- **Key Function**: `table(grammar: &Grammar) -> TokenStream`
- **Process**:
  1. Builds LR(0) table using `TableBuilder`
  2. Generates `Table` struct implementing `TableTrait`
  3. Generates action/expected/build_rule functions
- **Output**: Complete `Table` implementation with compile-time computed parsing table

#### `table/builder/builder.rs`
- **Purpose**: Core LR(0) table construction algorithm
- **Key Algorithm**:
  - **Closure**: Computes closure sets for non-terminals
  - **Follow**: Computes follow sets for non-terminals
  - **State Construction**: Builds LR(0) states using item sets
  - **Action Table**: Generates shift/reduce/goto actions
- **Key Methods**:
  - `closure()`: Computes closure of a non-terminal
  - `follow()`: Computes follow set of a non-terminal
  - `expand()`: Expands states by processing transitions
  - `build()`: Main entry point that constructs the complete table
- **State Representation**: Uses `State` (set of `StateItem`) to represent LR(0) states

#### `table/builder/item.rs`
- **Purpose**: LR(0) item representation
- **Key Type**: `StateItem` - represents a production rule with a dot position
- **Functionality**: Tracks which position in a rule we're currently parsing

#### `table/builder/state.rs`
- **Purpose**: LR(0) state representation
- **Key Type**: `State` - a set of `StateItem`s representing a parser state

#### `table/to_tokens/`
- **Purpose**: Convert table data structures to Rust code tokens
- **Key Modules**:
  - `build_fns/actions.rs`: Generates action lookup function
  - `build_fns/expected.rs`: Generates expected tokens function
  - `build_fns/build_rule.rs`: Generates AST construction function
- **Representations**: Various modules for converting data structures to token streams

#### `parser/mod.rs`
- **Purpose**: Generate `Parser` struct
- **Output**: Creates a `Parser` struct implementing `ParserTrait<Table>`

## Crate: `common` (Shared Types)

### Location: `/common/src/`

#### `lib.rs`
- **Purpose**: Re-exports all common modules

#### `table/mod.rs`
- **Purpose**: Core trait for parser tables
- **Key Trait**: `TableTrait`
  - `StartSymbol`: Associated type for the start symbol enum
  - `start_state()`: Returns initial parser state
  - `action(state, token)`: Returns parsing action (shift/reduce/goto)
  - `expected(state)`: Returns set of expected tokens for error messages
  - `build_rule(variant, children)`: Constructs AST node from variant ID and children
  - `alphabet()`: Returns set of terminal patterns

#### `table/state.rs`
- **Purpose**: State ID type
- **Type**: `StateId = usize`

#### `action.rs`
- **Purpose**: Parser action definitions
- **Action Types**:
  - `Action::Shift(StateId)`: Shift token and transition to new state
  - `Action::Reduce(VariantId)`: Reduce using a production rule
  - `Action::Goto(StateId)`: Transition to new state (after reduction)

#### `id/mod.rs`
- **Purpose**: Symbol ID representation
- **Key Types**:
  - `Id`: Enum representing terminal or non-terminal
    - `Id::T(Terminal)`: Terminal symbol
    - `Id::N(NonTerminal)`: Non-terminal symbol
  - `Terminal`: Terminal representation (labeled or EOF)
  - `NonTerminal`: Non-terminal representation (identifier)

#### `variants/mod.rs` and `variants/id.rs`
- **Purpose**: Variant identification
- **Key Type**: `VariantId`
  - Identifies a specific production rule
  - Contains: symbol, variant name, and rule length
  - Used for AST construction during reduction

## Data Flow

### Compile-Time (Macro Expansion)

1. **Grammar Parsing**: `build_parser!` macro input is parsed into `Grammar` structure
2. **Enum Generation**: For each non-terminal, generate Rust enum with variants
3. **Table Construction**:
   - Build LR(0) closure sets
   - Compute follow sets
   - Construct LR(0) states
   - Generate action table (shift/reduce/goto)
4. **Code Generation**: Convert table data to Rust code implementing `TableTrait`
5. **Parser Generation**: Generate `Parser` struct implementing `ParserTrait<Table>`

### Runtime (Parsing)

1. **Lexing**: Input string is tokenized using regex patterns from alphabet
2. **Initialization**: `ParseInstance` is created with tokenized input
3. **Parsing Loop**:
   - Look up action for current state and lookahead token
   - **Shift**: Push token onto stack, advance state
   - **Reduce**: Pop children from stack, build AST node, push result
   - **Goto**: Transition to new state after reduction
4. **Completion**: When start symbol is on stack and input is consumed, return result

## Key Algorithms

### LR(0) Table Construction

The `TableBuilder` implements standard LR(0) table construction:

1. **Closure Computation**: For a non-terminal, include all its production rules and recursively include closures of non-terminals that appear at the start of those rules
2. **Follow Set Computation**: For each non-terminal, find all symbols that can follow it in any production
3. **State Construction**: Start with closure of start symbol, then expand states by processing transitions
4. **Action Generation**: For each state and lookahead:
   - If dot before terminal → Shift action
   - If dot at end of rule → Reduce action
   - If dot before non-terminal → Goto action

### Parsing Algorithm

Standard LR(0) shift-reduce parsing:
- Maintain state stack and value stack
- On shift: push state and token value
- On reduce: pop rule length items, build AST node, push result
- On goto: transition to new state

## File Organization

```
lr_parser/
├── Cargo.toml                 # Main crate configuration
├── src/                       # Main library source
│   ├── lib.rs
│   ├── parser/
│   ├── lexer/
│   └── tokens/
├── macros/                    # Procedural macro crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── grammar/
│       ├── enums/
│       ├── table/
│       └── parser/
├── common/                    # Shared types
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── table/
│       ├── action.rs
│       ├── id/
│       └── variants/
├── tests/                     # Integration tests
├── examples/                  # Example usage
└── README.md
```

## Testing

- **Integration Tests**: Located in `/tests/`
  - `full.rs`: Comprehensive grammar tests
  - `other.rs`: Additional test cases
  - `table_macro.rs`: Table generation tests
- **Unit Tests**: Located within each crate's source files
- **Grammar Tests**: Located in `macros/src/grammar/tests/`

## Dependencies

- **`syn`**: Parsing Rust code in procedural macros
- **`quote`**: Generating Rust code in procedural macros
- **`proc_macro2`**: Procedural macro utilities
- **`regex`**: Terminal pattern matching

## Error Handling

- **Parse Errors**: Generated when unexpected tokens are encountered
- **Lex Errors**: Generated when input cannot be tokenized
- **Alphabet Errors**: Generated when regex patterns are invalid
- All errors are unified in `lr_parser::Error` enum`

## Feature Flags

- **`manual_lexing`**: Exposes lexer module for manual tokenization control

## Important Notes

1. **Start Symbol**: Grammar must always define a non-terminal named `S` as the start symbol
2. **Terminal Patterns**: Use regex syntax, escape special characters (e.g., `"\\+"` for literal `+`)
3. **Type Safety**: AST types are generated at compile-time, ensuring type safety
4. **Zero Runtime Overhead**: Table construction happens entirely at compile-time
5. **LR(0) Limitations**: This is an LR(0) parser, meaning it doesn't use lookahead. Some grammars may require LR(1) or LALR(1) for proper parsing

