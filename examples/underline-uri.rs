#![expect(missing_docs)]
use difference_rs::{Changeset, ChangesetMulti, Difference};
use std::{char::REPLACEMENT_CHARACTER, io::Write};

// Screenshot:
// https://raw.githubusercontent.com/naomijub/difference-rs/master/assets/word-underline.png

#[allow(unused_must_use)]
fn main() {
    let uri_1 = "https://localhost:8080/path?query=value";
    let uri_2 = "https://myapi.com/api/path?query=asset";

    let mut t = term::stdout().unwrap();

    let ChangesetMulti {
        diffs,
        splits,
        edit_splits,
        ..
    } = Changeset::new_multi(uri_1, uri_2, &["://", "/", "?", "="]);

    let mut orig_counter = 0;
    let mut edit_counter = 0;
    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                let orig = z.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                for word in orig {
                    orig_counter += word.len();
                    if let Some(split) = splits.iter().find(|(idx, _split)| idx == &orig_counter) {
                        orig_counter += split.1.len();
                        t.fg(term::color::RED).unwrap();
                        write!(t, "{word}{}", split.1).unwrap();
                    } else {
                        t.fg(term::color::RED).unwrap();
                        write!(t, "{word}");
                    }
                }
            }
            Difference::Rem(ref z) => {
                let orig = z.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                for word in orig {
                    orig_counter += word.len();
                    if let Some(split) = splits.iter().find(|(idx, _split)| idx == &orig_counter) {
                        orig_counter += split.1.len();
                        t.fg(term::color::WHITE).unwrap();
                        t.bg(term::color::RED).unwrap();
                        write!(t, "{}{}", word, split.1).unwrap();
                    } else {
                        t.fg(term::color::WHITE).unwrap();
                        t.bg(term::color::RED).unwrap();
                        write!(t, "{word}").unwrap();
                    }
                }
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
                let edit = z.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                for word in edit {
                    edit_counter += word.len();
                    if let Some(split) = edit_splits
                        .iter()
                        .find(|(idx, _split)| idx == &edit_counter)
                    {
                        edit_counter += split.1.len();
                        t.fg(term::color::GREEN).unwrap();
                        write!(t, "{word}{}", split.1).unwrap();
                    } else {
                        t.fg(term::color::GREEN).unwrap();
                        write!(t, "{word}");
                    }
                }
            }
            Difference::Add(ref z) => {
                let edit = z.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                for word in edit {
                    edit_counter += word.len();
                    if let Some(split) = edit_splits
                        .iter()
                        .find(|(idx, _split)| idx == &edit_counter)
                    {
                        edit_counter += split.1.len();
                        t.fg(term::color::WHITE).unwrap();
                        t.bg(term::color::GREEN).unwrap();
                        write!(t, "{}{}", word, split.1);
                    } else {
                        t.fg(term::color::WHITE).unwrap();
                        t.bg(term::color::GREEN).unwrap();
                        write!(t, "{word}");
                    }
                }
                t.reset().unwrap();
            }
            Difference::Rem(_) => (),
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
}
