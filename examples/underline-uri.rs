#![expect(missing_docs)]
use difference_rs::{Changeset, ChangesetMulti, Difference};
use std::io::Write;

// Screenshot:
// https://raw.githubusercontent.com/naomijub/difference-rs/master/assets/word-underline.png

#[allow(unused_must_use)]
fn main() {
    let uri_1 = "https://localhost:8080/path?query=value";
    let uri_2 = "https://myapi.com/api/path?query=asset";

    let mut t = term::stdout().unwrap();

    let ChangesetMulti { diffs, .. } = Changeset::new_multi(uri_1, uri_2, &["://", "/", "?", "="]);

    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                t.fg(term::color::RED).unwrap();
                write!(t, "{z}");
            }
            Difference::Rem(ref z) => {
                t.fg(term::color::WHITE).unwrap();
                t.bg(term::color::RED).unwrap();
                write!(t, "{z}").unwrap();
                t.reset().unwrap();
            }
            Difference::Add(_) => (),
        }
    }
    t.reset().unwrap();

    writeln!(t);

    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                t.fg(term::color::GREEN).unwrap();
                write!(t, "{z}");
            }
            Difference::Add(ref z) => {
                t.fg(term::color::WHITE).unwrap();
                t.bg(term::color::GREEN).unwrap();
                write!(t, "{z}");
                t.reset().unwrap();
            }
            Difference::Rem(_) => (),
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
}
