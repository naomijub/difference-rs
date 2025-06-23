# difference_rs [![](https://travis-ci.org/naomijub/difference_rs.svg?branch=master)](https://travis-ci.org/naomijub/difference_rs) [![](https://ci.appveyor.com/api/projects/status/fgfp1mw85ffkl9bu/branch/master?svg=true)](https://ci.appveyor.com/project/naomijub/difference-rs/branch/master) [![](https://coveralls.io/repos/naomijub/difference_rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/naomijub/difference_rs?branch=master) [![](https://img.shields.io/crates/v/difference.svg)](https://crates.io/crates/difference)
A Rust text diffing library with built-in diffing assertion.

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

![](https://raw.githubusercontent.com/naomijub/difference_rs/master/assets/fox.png)
![](https://raw.githubusercontent.com/naomijub/difference_rs/master/assets/github-style.png)

Usage
----------

Add the following to your Cargo.toml:

```toml
[dependencies]
difference_rs = "3.0"
```

Now you can use the crate in your code

Using the binary
-----------------

difference_rs can also be used as a command-line application. The best way to install it is using:

```sh
$ cargo install --features=bin
```
