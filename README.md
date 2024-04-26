# Nile tools

This repository contains various of tools to help `nile` with importing/exporting OpenTTD's language files.

## Importing english

For `english`, it simply converts the OpenTTD `.txt` format into `JSON`, by splitting up the string and English text.
Additionally, it uses `git blame` to find the "version" of each string (where "version" here is the hash of the commit that last changed the string).

This importing can be done on regular intervals, to always feed `nile` the latest `english`.

## Importing other languages

For other languages, it does the same, except for "version".
Here it uses `git blame` to find which commit changed the translation, then uses `git blame` on the `english.txt` to find the "version" of the base string.
This is a good approximation of what version of the string was translated.

This is normally only done once, when a new project is switched over to `nile`.

## Installation

Have Rust [installed](https://www.rust-lang.org/tools/install).

## Development

```bash
cargo run -- -p <path-to-OpenTTD> <language>
```
