# Typst as Library

Check out the original [tfachmann/typst-as-library](https://github.com/tfachmann/typst-as-library).

Quote from its README:

> This repository shows how to use [typst](https://github.com/typst/typst) as a library in Rust.
> Any code presented in this repository is meant to help you understand how to interface with `typst`.
> Please use the code as you like.

## This fork: Minimal example

The goal of this fork is to simply render a "Hello, World!" string with typst. For that, we don't need
some of the functionality which is implemented in the original `typst-as-library`, e.g., system fonts,
loading plugins from an online source,
and mainly that the inclusion of typst files works.

Instead a binary crate is provided, which lets you compile a document start-to-finish by simply running

```
cargo run
```

---

## Acknowledgment

Code has been inspired by
- [https://github.com/tfachmann/typst-as-library](https://github.com/tfachmann/typst-as-library).
- [https://github.com/fenjalien/obsidian-typst](https://github.com/fenjalien/obsidian-typst)
- [https://github.com/mattfbacon/typst-bot](https://github.com/mattfbacon/typst-bot)
