# difference_rs 

[![License:MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/naomijub/difference-rs/actions/workflows/rust.yaml/badge.svg?branch=master)](https://github.com/naomijub/difference-rs/actions/workflows/rust.yaml)
[![Coverage Status](https://coveralls.io/repos/github/naomijub/difference-rs/badge.svg)](https://coveralls.io/github/naomijub/difference-rs)

A Rust, updated, text diffing library with built-in diffing assertion.

__[Examples](/Examples.md)__

```rust
use difference_rs::Changeset;

let changeset = Changeset::new("test", "tent", "");

assert_eq!(changeset.diffs, vec![
  Difference::Same("te".to_string()),
  Difference::Rem("s".to_string()),
  Difference::Add("n".to_string()),
  Difference::Same("t".to_string())
]);
```

![](https://raw.githubusercontent.com/naomijub/difference-rs/master/assets/fox.png)
![](https://raw.githubusercontent.com/naomijub/difference-rs/master/assets/github-style.png)

```rust
use difference_rs::{Changeset, Difference};

let changeset = Changeset::new_multi(
   "https://localhost:8080/path?query=value",
   "https://myapi.com/api/path?query=asset",
   &["://", "/", "?", "="],
);

assert_eq!(changeset.diffs, vec![
    Difference::Same("https://".to_string()),
    Difference::Rem("localhost:8080/".to_string()),
    Difference::Add("myapi.com/api/".to_string()),
    Difference::Same("path?query=".to_string()),
    Difference::Rem("value".to_string()),
    Difference::Add("asset".to_string()),
]);
```

![](https://raw.githubusercontent.com/naomijub/difference-rs/master/assets/uri-underline.png)

Usage
----------

Add the following to your Cargo.toml:

```toml
[dependencies]
difference_rs = "3.1"
```

Now you can use the crate in your code

Using the binary
-----------------

difference_rs can also be used as a command-line application. The best way to install it is using:

```sh
$ cargo install --features=bin
```
