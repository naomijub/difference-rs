#![expect(missing_docs)]
use difference_rs::{Changeset, Difference};
use std::io::Write;

// Screenshot:
// https://raw.githubusercontent.com/naomijub/difference-rs/master/assets/word-underline.png

#[allow(unused_must_use)]
fn main() {
    let text1 = "Roses are red, violets are blue.";
    let text2 = "Roses are blue, violets are";

    let mut t = term::stdout().unwrap();

    let Changeset { diffs, .. } = Changeset::new(text1, text2, "");

    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                t.fg(term::color::RED).unwrap();
                write!(t, "{z}");
            }
            Difference::Rem(ref z) => {
                t.fg(term::color::WHITE).unwrap();
                t.bg(term::color::RED).unwrap();
                write!(t, "{z}");
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
