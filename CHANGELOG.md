# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Operator precedence and left associativity for all binary operators, so chained expressions
  like `1 + 2 * 3` and `a < b && c` parse
- Parentheses for grouping, for example `(2 + 3) * 4` and `!(a == b)`
- `#` line comments; the REPL ignores empty and comment-only lines
- Floor division `//` for floats
- Tests for the new parsing behaviour
- MIT license

### Fixed
- `if` and `while` conditions and `for` iterables accept any expression, not only a literal or
  variable (`if x > 5 { … }`, `for i in range(3) { … }`)
- Nested parentheses in function call arguments (`add(add(1, 2), 3)`)
- `&&` and `||` short-circuit instead of always evaluating both sides
- `//` rounds down for negative operands (`-7 // 2` is `-4`, was `-3`)
- Identifiers starting with `true` or `false` (such as `trueish`) are no longer split into a
  boolean and a leftover name

### Changed
- README rewritten with a quick start, a language tour of implemented features, known
  limitations, project layout, development commands and roadmap
- `docs/documentation.md` marked as a design specification that is not yet implemented
- `TEST_COVERAGE.md` counts corrected and last verified run recorded
- Added `CONTRIBUTING.md`, a pull request template and issue templates for bug reports and
  language proposals

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
