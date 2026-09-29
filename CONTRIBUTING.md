# Contributing to Nara

Thanks for your interest in Nara! This guide covers how to set up the project, how the code is
organized, and what a good pull request looks like.

## Ways to contribute

- **Report a bug.** Open an issue with the input you typed, what you expected and what you got.
- **Propose a language feature.** Open an issue first. Syntax and semantics are still being
  decided, so a short discussion before code saves everyone time.
- **Fix a bug or implement a roadmap item.** See the [roadmap](README.md#roadmap) and
  [known limitations](README.md#known-limitations) in the README.
- **Improve docs and tests.** Examples, clearer error messages and missing edge-case tests are
  always welcome.

## Getting started

You need a Rust toolchain with edition 2024 support (Rust 1.85+).

```bash
git clone https://github.com/neonid0/nara.git
cd nara
make test       # run the test suite
make run        # start the REPL
make help       # list every make target
```

## How the code is organized

The interpreter lives in `crates/nara`; `crates/nara-cli` is a thin REPL around it.

- **Parsing** is done with small hand-written combinators in `utils.rs` (`tag`,
  `extract_whitespace`, `extract_ident`, `sequence`, …). There is no separate lexer.
- **Every AST node** exposes `new(s: &str) -> Result<(&str, Self), String>`, which parses from
  the start of `s` and returns the unconsumed rest, and an `eval` method that computes a `Val`.
- **Expressions** (`expression.rs`) are parsed by precedence climbing: `Expression::new` calls
  `new_binary`, which calls `new_primary` for literals, calls, `if`/`while`/`for`, lists,
  blocks and parenthesized groups. Operator precedence is defined in `Op::precedence`.
- **Statements** (`statement.rs`) are `val` bindings, `fn` definitions or expressions.
- **Environments** (`env.rs`) form a parent chain for scoping; strings go through the
  `StringInterner`.

### Adding a feature

1. Add the syntax to the relevant parser, usually a new variant in `Expression` and a branch
   in `new_primary`, or a new operator in `Op` with its precedence.
2. Handle the new variant in `eval`.
3. Add tests (see below).
4. Update the [language tour](README.md#language-tour) and the
   [`Unreleased`](CHANGELOG.md) section of the changelog.

## Tests

Tests sit next to the code in each module's `#[cfg(test)] mod tests`. Follow the existing
naming so tests are easy to find:

- `parse_*` checks the AST a parser produces
- `eval_*` checks the value an AST evaluates to
- `test_*` in `lib.rs` runs source text end to end through `parse(...).eval(...)`

Add at least one parse test and one eval test for new syntax, plus error cases where they make
sense. If you add a new feature area, list it in [`TEST_COVERAGE.md`](TEST_COVERAGE.md).

## Before opening a pull request

Run the full check, which formats, lints, tests and builds:

```bash
make all
```

There is no CI yet, so `make all` is the bar: `cargo fmt` leaves no diff, `cargo clippy`
passes with `-D warnings`, and every test passes.

## Commit messages

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) on a single line:

```text
feat(lang): add while loops
fix(parser): accept expressions as if conditions
docs: document operator precedence
test: cover floor division with negative numbers
```

Common types are `feat`, `fix`, `docs`, `test`, `refactor`, `build` and `chore`. Use a scope
such as `lang`, `core`, `parser` or `cli` when it helps.

## Pull requests

- Keep each pull request focused on one change.
- Describe what changed and why, and link the issue it closes (`Closes #12`).
- Include a REPL example for user-visible changes.
- Update the README and changelog when behaviour changes.

## Releases

Versions follow [Semantic Versioning](https://semver.org/). Maintainers bump the version with
`make bump-patch`, `make bump-minor` or `make bump-major` (requires `cargo-edit`), run
`make release-workflow`, and tag with `make tag-release`.
