# Nara

Nara is an experimental programming language written in Rust. This repository contains a
tree-walking interpreter (`nara` crate) built on a hand-written combinator parser, and an
interactive REPL (`nara-cli`).

> **Status:** v0.2.0, experimental. The interpreter implements the core expression language
> described below. The long-term design (linear types, actors, capabilities, agent primitives)
> lives in [`docs/documentation.md`](docs/documentation.md) and is **not implemented yet**.

## Quick start

Requires a Rust toolchain with edition 2024 support (Rust 1.85+).

```bash
git clone https://github.com/neonid0/nara.git
cd nara
make run        # start the REPL (same as: cargo run -q --bin nara-cli)
```

```text
-> val name = "Nara"
-> f"Hello {name}!"
String("Hello Nara!")
-> fn add(a, b) { a + b }
-> add(2, 40)
Number(42)
```

Each line you enter is parsed and evaluated in a persistent environment, so bindings and
functions stay available for later lines. Press `Ctrl-D` to exit.

## Language tour

Everything in this section runs on the current interpreter. `#` starts a comment that runs to
the end of the line; here the comments show the REPL output.

### Values

| Type     | Examples                    |
| -------- | --------------------------- |
| Number   | `42`, `-7` (64-bit integer) |
| Float    | `3.14`, `-0.5`              |
| String   | `"hello\n"`                 |
| Bool     | `true`, `false`             |
| List     | `[1, "two", true]`          |
| Function | `fn double(x) { x + x }`    |
| Unit     | result of statements        |

### Bindings and blocks

```text
val x = 10; val y = 3; x + y     # Number(13)
{ val a = 1; val b = 2; a + b }  # Number(3), a and b are scoped to the block
```

Several statements can share one line when separated by `;`. The value of the last one is
returned.

### Operators

| Kind       | Operators                        |
| ---------- | -------------------------------- |
| Arithmetic | `+` `-` `*` `/` `//` (floor div) |
| Comparison | `==` `!=` `<` `>` `<=` `>=`      |
| Logical    | `&&` `\|\|` `!`                  |
| Unary      | `-` (negation), `!` (not)        |

Precedence from lowest to highest: `||`, `&&`, `==` `!=`, `<` `<=` `>` `>=`, `+` `-`,
`*` `/` `//`. Binary operators are left-associative, parentheses group, and `&&`/`||`
short-circuit.

```text
2 + 3 * 4                        # Number(14)
(2 + 3) * 4                      # Number(20)
10 - 4 - 3                       # Number(3)
-7 // 2                          # Number(-4), floor division rounds down
!(1 == 2) && 3 >= 3              # Bool(true)
```

`+` also concatenates strings: `"con" + "cat"` → `String("concat")`.

### Strings and f-strings

```text
val name = "Nara"
f"Hello {name}!"                 # String("Hello Nara!")
f"{2 * 21} is the answer"        # String("42 is the answer")
val xs = [1, 2, 3]
print(f"list: {xs}")             # prints: list: [1, 2, 3]
```

Identical strings are interned, so repeated literals share one allocation.

### Control flow

```text
val x = 10
if x > 5 { "big" } else { "small" }     # String("big")
if x > 50 { 1 } else if x > 5 { 2 } else { 3 }   # Number(2)

for i in range(3) { print(i * i) }      # prints 0, 1, 4
for s in ["a", "b"] { print(s) }        # prints a, b
while x < 5 { 1 }                       # Unit
```

`if`, `while` and `for` are expressions: they return the value of the branch or last
iteration, or `Unit`.

### Functions

```text
fn double(x) { x + x }
double(21)                       # Number(42)
fn add(a, b) { a + b }
add(add(1, 2), 3 * 2)            # Number(9)
```

Built-in functions:

| Function         | Description                                   |
| ---------------- | --------------------------------------------- |
| `print(value)`   | Write a value to stdout                       |
| `len(value)`     | Length of a string or list                    |
| `range(n)`       | List `[0, 1, …, n-1]`                         |
| `range(a, b)`    | List `[a, a+1, …, b-1]`                       |

### Truthiness

`false`, `0`, `""` and `[]` are falsy; every other value is truthy.

## Known limitations

- **No reassignment.** Bindings are immutable (`val`); `mut` exists only in the design docs, so
  a `while` loop whose condition starts out true never ends.
- **No implicit numeric conversion.** `1 + 2.0` is a type error; use `1.0 + 2.0`.
- **Single-line input.** The REPL evaluates one line at a time, so multi-line programs must be
  joined with `;`.
- **Newlines are not statement separators on their own.** A line that starts with `-` continues
  the previous expression (`x\n-1` is `x - 1`); separate statements with `;`.

## Project layout

```text
crates/
  nara/          # library: parser, AST, evaluator
    src/
      expression.rs        # expressions, operators, if/while/for, calls, lists, f-strings
      expression/block.rs  # block expressions and scoping
      statement.rs         # statements (val, fn, expressions)
      binding_def.rs       # `val` bindings
      function_def.rs      # `fn` definitions
      env.rs               # environments with parent scopes
      interner.rs          # string interner
      utils.rs             # parser combinators
      val.rs               # runtime values
  nara-cli/      # REPL binary
docs/
  documentation.md         # long-term language design (not yet implemented)
syntax.nara                # planned surface syntax (design notes)
tokens.nara                # planned keywords and tokens
```

## Development

| Command              | What it does                                  |
| -------------------- | --------------------------------------------- |
| `make run`           | Start the REPL                                |
| `make test`          | Run the test suite (154 tests)                |
| `make clippy`        | Lint with `-D warnings`                       |
| `make fmt`           | Format the code                               |
| `make all`           | fmt, clippy, test and build                   |
| `make release`       | Optimized build                               |
| `make install`       | Install `nara-cli` with `cargo install`       |
| `make help`          | List every target, including version helpers  |

See [`TEST_COVERAGE.md`](TEST_COVERAGE.md) for what the tests cover and
[`CHANGELOG.md`](CHANGELOG.md) for release notes.

## Roadmap

_Parser_
- Block expression return statement
- Multi-line input in the REPL

_Syntax_
- Decide semicolon and return usage (optional or not)
- `nil`, `null` or `none`
- Mutable bindings (`mut`) and reassignment

_Extra_
- Method usage (`anArray.map()`)
- `type`, `enum`, `proto`, `impl`
- `public`, `private`, `protected`, `static`, `abstract`, `final`

## Contributing

Bug reports, language proposals and pull requests are welcome. Read
[`CONTRIBUTING.md`](CONTRIBUTING.md) for setup, code layout, testing conventions and commit
style.

## Motivation

Build a programming language for building an operating system, focusing on simplicity,
performance, and ease of use. The language should be suitable for systems programming, with a
strong emphasis on low-level operations while maintaining high-level abstractions for ease of
development.

## License

Nara is released under the [MIT License](LICENSE).
