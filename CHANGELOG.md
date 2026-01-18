# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-01-19

### Added
- Boolean type with `true` and `false` literals
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical operators: `&&` (and), `||` (or), `!` (not)
- Unary negation operator `-` for numbers and floats
- If/else expressions with optional else-if chaining
- While loops for conditional iteration
- For loops for iterating over lists
- List/array type with literal syntax `[1, 2, 3]`
- Function evaluation and calling with parameter binding
- Built-in functions:
  - `print()` - output values to stdout
  - `len()` - get length of strings and lists
  - `range()` - generate numeric ranges
- Truthiness evaluation for all value types
- List display in f-string interpolation

### Changed
- Val enum now includes Bool, Function, and List variants
- Expression enum supports If, While, For, UnaryOp, List, and FunctionCall
- Function definitions now store and evaluate properly
- All types now derive Clone for function body storage

### Technical Details
- Added `is_truthy()` method to Val for conditional evaluation
- Implemented child environment creation for loop variable scoping
- Function bodies stored as `Rc<Statement>` for efficient cloning
- List elements evaluated lazily during list literal parsing

## [0.1.0] - 2026-01-19

### Added
- Multi-statement parsing on single line (e.g., `val x = 10; x`)
- String concatenation with `+` operator
- String interning for memory efficiency and deduplication
- F-string interpolation syntax: `f"Hello {name}!"` with expression evaluation
- Makefile for build automation with common development tasks
- Proper semantic versioning and git tagging system
- Comprehensive test suite with 81 tests covering all features
- Initial release
- Basic expression evaluation (numbers, floats, strings)
- Variable bindings with `val` keyword
- Function definitions with `fn` keyword
- Block expressions with scoped environments
- Arithmetic operations: `+`, `-`, `*`, `/`, `//` (floor division)
- String literals with escape sequences
- REPL interface via nara-cli
- Combinator-based parser architecture

### Changed
- Operation expressions now support any expression type (not just numbers)
- Parse struct now holds Vec<Statement> instead of single statement
- Environment now includes StringInterner for efficient string management

### Technical Details
- Refactored `Operation` to use `Box<Expression>` for operands
- Added `RefCell<StringInterner>` to `Env` for interior mutability
- Implemented type dispatch in operation evaluation
- Added `FStringPart` enum for f-string parsing
