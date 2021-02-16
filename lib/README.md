# Benvolio

> Run all the doc tests in files matching a specified glob pattern.

Heavily inspired by [doc_comment](https://github.com/GuillaumeGomez/doc-comment/).

```rust
#[macro_use]
extern crate benvolio;

doctest_glob!("doctests/*.md");
```

If you only want to run these tests when running doctests you should do something like the below.

```rust
#[macro_use]
extern crate benvolio;

#[cfg(doctest)]
mod doctests {
    doctest_glob!("doctests/*.md");
}
```
