# Contributing to Vedic

Thanks for helping improve Vedic. The language runtime lives in Rust in this repository. The tutorial site, browser IDE, and VS Code extension are maintained in their own repositories in the `vedic-lang` organization.

## Development setup

Install the stable Rust toolchain, Python 3, and the nightly Rust toolchain (for formatting). From the `vedic` repository root:

```bash
cargo build --workspace
cargo test --workspace
python tools -bt
```

The Python command builds the CLI and runs the `.ved` integration programs under `tests/`. Formatting can be checked with:

```bash
cargo +nightly fmt --all -- --check
```

## Contributions

Useful contributions include bug fixes, focused test cases, clearer errors, documentation improvements, and small features that fit the language's direction.

When changing language behavior, add a focused `.ved` case under `tests/` and update the relevant tutorial page in `vedic-lang.github.io` if user-facing syntax or behavior changes. Please run the checks above before opening a pull request.

Open bug reports and feature requests in the [Vedic issue tracker](https://github.com/vedic-lang/vedic/issues). Include a small source example, the command or steps used, expected behavior, and actual output.
