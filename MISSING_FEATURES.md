# Missing Functionalities & Known Issues

This document tracks features that are planned but not yet implemented, as well as known limitations in the current implementation.

## Parser Limitations

### 1. Operator Precedence
**Status**: ❌ Not Implemented  
**Impact**: Critical  
**Issue**: Complex expressions with mixed operators may not parse correctly.

**Examples that fail**:
```nara
// This fails to parse:
fn factorial(n) { n * factorial(n - 1) }

// Workaround: Use intermediate variables
fn factorial(n) {
  if n <= 1 {
    1
  } else {
    val next = factorial(n - 1);
    n * next
  }
}
```

**Root Cause**: The parser doesn't have proper operator precedence tables. Binary operations are parsed left-to-right without considering precedence.

**Planned Fix**: v0.3.0 - Implement Pratt parser or precedence climbing algorithm.

### 2. Comment Support
**Status**: ⚠️ Partial  
**Impact**: Medium  
**Issue**: Line comments (`//`) are not properly supported in all contexts.

**Current State**:
- Comments work in REPL
- Comments in files may cause parsing issues
- Comments conflict with floor division operator (`//`)

**Examples**:
```nara
// This works in REPL but may fail in files
val x = 10;  // This is a comment

// Floor division conflicts with comments
3 // 4  // Ambiguous: floor division or comment?
```

**Planned Fix**: v0.3.0 - Implement proper comment tokenization before parsing.

### 3. Multi-line File Support
**Status**: ⚠️ Limited  
**Impact**: High  
**Issue**: Files with multiple lines and statements don't parse correctly.

**Current Workaround**: Write scripts on single lines with semicolons:
```nara
val x = 10; val y = 20; print(x + y)
```

**Planned Fix**: v0.3.0 - Improve statement separator handling and whitespace parsing.

### 4. Parenthesized Expressions
**Status**: ❌ Not Implemented  
**Impact**: High  
**Issue**: Cannot use parentheses to control evaluation order.

**Examples that don't work**:
```nara
val result = (1 + 2) * 3;  // Fails to parse
val x = (a + b) / (c + d); // Fails to parse
```

**Workaround**: Use intermediate variables:
```nara
val sum1 = 1 + 2;
val result = sum1 * 3;
```

**Planned Fix**: v0.3.0 - Add parenthesis support in expression parser.

## Missing Language Features

### 1. Return Statements
**Status**: ❌ Not Implemented  
**Impact**: Medium  
**Issue**: Cannot explicitly return from middle of function.

**Current Behavior**: Last expression in block is returned implicitly.

**Planned Fix**: v0.3.0

### 2. Break/Continue
**Status**: ❌ Not Implemented  
**Impact**: Medium  
**Issue**: Cannot break out of loops early or skip iterations.

**Workaround**: Use conditional logic:
```nara
for i in range(10) {
  if i > 5 {
    // Can't break, will continue to 10
  }
}
```

**Planned Fix**: v0.3.0

### 3. Pattern Matching
**Status**: ❌ Not Implemented  
**Impact**: Medium  
**Syntax**: `match` expressions for destructuring and matching.

**Planned Fix**: v0.4.0

### 4. Tuples
**Status**: ❌ Not Implemented  
**Impact**: Medium  
**Syntax**: `(a, b, c)` for grouping values.

**Planned Fix**: v0.3.0

### 5. Structs/Records
**Status**: ❌ Not Implemented  
**Impact**: High  
**Syntax**: Custom data types.

**Planned Fix**: v0.4.0

### 6. Enums
**Status**: ❌ Not Implemented  
**Impact**: High  
**Syntax**: Sum types for representing variants.

**Planned Fix**: v0.4.0

### 7. Methods
**Status**: ❌ Not Implemented  
**Impact**: High  
**Syntax**: `object.method()` calling.

**Example**:
```nara
// Not yet supported:
val text = "hello";
val upper = text.to_upper();
val length = myList.len();
```

**Workaround**: Use functions:
```nara
val length = len(myList);
```

**Planned Fix**: v0.3.0

### 8. Module System
**Status**: ❌ Not Implemented  
**Impact**: Critical (for larger programs)  
**Syntax**: `import`, `export`, module declarations.

**Planned Fix**: v0.4.0

### 9. Error Handling
**Status**: ❌ Not Implemented  
**Impact**: High  
**Issue**: No `Result` or `Option` types for error handling.

**Current Behavior**: Runtime errors crash the program.

**Planned Fix**: v0.4.0

### 10. Generics
**Status**: ❌ Not Implemented  
**Impact**: High  
**Syntax**: Generic functions and types.

**Planned Fix**: v0.5.0

## Type System Limitations

### 1. Type Annotations
**Status**: ❌ Not Implemented  
**Issue**: Cannot explicitly declare types.

**Example (future syntax)**:
```nara
val x: Int = 10;
fn add(a: Int, b: Int) -> Int { a + b }
```

**Planned Fix**: v0.4.0

### 2. Type Inference
**Status**: ⚠️ Basic  
**Issue**: Types are inferred at runtime only.

**Planned Fix**: v0.4.0 - Static type inference.

## Standard Library Gaps

### Currently Implemented
- `print(x)` - Print value
- `len(list)` - Get list/string length
- `range(n)` - Generate range 0..n

### Missing (High Priority)
- String methods: `split`, `join`, `trim`, `substring`
- List methods: `map`, `filter`, `reduce`, `push`, `pop`
- Math functions: `abs`, `min`, `max`, `floor`, `ceil`
- I/O: `read`, `write`, file operations
- Type conversion: `to_string`, `to_int`, `to_float`

**Planned Fix**: Ongoing through v0.3.0 - v0.5.0

## CLI & Tooling

### 1. REPL History
**Status**: ❌ Not Implemented  
**Issue**: Cannot use arrow keys to recall previous commands.

**Planned Fix**: v0.3.0 - Use rustyline for better REPL.

### 2. Error Messages
**Status**: ⚠️ Basic  
**Issue**: Error messages don't show line numbers or context.

**Current**:
```
Parse error: input was not consumed fully by parser
```

**Desired**:
```
Parse error at line 5, column 10:
  val x = 10 *
             ^
Expected expression after operator
```

**Planned Fix**: v0.3.0

### 3. Debugger
**Status**: ❌ Not Implemented  
**Planned Fix**: v1.0.0

### 4. LSP (Language Server Protocol)
**Status**: ❌ Not Implemented  
**Planned Fix**: v1.0.0

## Performance Issues

### 1. No Optimization
**Status**: ⚠️ Unoptimized  
**Issue**: AST is interpreted directly without optimization passes.

**Planned Fix**: v0.5.0 - Add basic optimization passes.

### 2. Memory Management
**Status**: ⚠️ Basic  
**Issue**: String interning is implemented, but no advanced memory pooling.

**Planned Fix**: v0.5.0

## Documentation Gaps

### Missing
- Language specification
- API documentation for library use
- Tutorial series
- Cookbook/examples repository

**Planned Fix**: Ongoing

## Testing Gaps

### Current Coverage
- Parser: ✅ Good
- Evaluator: ⚠️ Partial
- Edge cases: ❌ Limited

### Missing
- Fuzzing tests
- Performance benchmarks
- Integration tests for file execution
- Error handling tests

**Planned Fix**: v0.3.0

## Roadmap Priority

### v0.3.0 (Next Release)
1. Fix operator precedence parsing
2. Add parenthesis support
3. Implement proper comment handling
4. Add return/break/continue statements
5. Implement tuples
6. Add method call syntax
7. Improve error messages

### v0.4.0
1. Type system foundations
2. Structs and enums
3. Pattern matching
4. Module system
5. Error handling (Result/Option)

### v0.5.0
1. Generics
2. Performance optimizations
3. Advanced standard library

### v1.0.0
1. Self-hosting compiler
2. Debugger
3. LSP support
4. Complete standard library
5. OS development toolkit

---

**Last Updated**: 2025-01-19  
**Current Version**: v0.2.0
