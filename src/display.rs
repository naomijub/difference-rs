use crate::ChangesetMulti;

use super::{Changeset, Difference};
use std::{char::REPLACEMENT_CHARACTER, fmt};

impl fmt::Display for Changeset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in &self.diffs {
            match *d {
                Difference::Same(ref x) => {
                    write!(f, "{}{}", x, self.split)?;
                }
                Difference::Add(ref x) => {
                    write!(f, "\x1b[92m{}\x1b[0m{}", x, self.split)?;
                }
                Difference::Rem(ref x) => {
                    write!(f, "\x1b[91m{}\x1b[0m{}", x, self.split)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for ChangesetMulti {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut orig_counter = 0;
        let mut edit_counter = 0;
        for d in &self.diffs {
            match *d {
                Difference::Same(ref x) => {
                    let orig = x.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                    for word in orig {
                        orig_counter += word.len();
                        edit_counter += word.len();
                        if let Some(split) = self
                            .splits
                            .iter()
                            .find(|(idx, _split)| idx == &orig_counter)
                        {
                            orig_counter += split.1.len();
                            edit_counter += split.1.len();
                            write!(f, "{}{}", word, split.1)?;
                        } else {
                            write!(f, "{word}")?;
                        }
                    }
                }
                Difference::Add(ref x) => {
                    let edit = x.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                    for word in edit {
                        edit_counter += word.len();
                        if let Some(split) = self
                            .edit_splits
                            .iter()
                            .find(|(idx, _split)| idx == &edit_counter)
                        {
                            edit_counter += split.1.len();
                            write!(f, "\x1b[92m{}\x1b[0m{}", word, split.1)?;
                        } else {
                            write!(f, "\x1b[92m{word}\x1b[0m")?;
                        }
                    }
                }
                Difference::Rem(ref x) => {
                    let orig = x.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                    for word in orig {
                        orig_counter += word.len();
                        if let Some(split) = self
                            .splits
                            .iter()
                            .find(|(idx, _split)| idx == &orig_counter)
                        {
                            orig_counter += split.1.len();
                            write!(f, "\x1b[91m{}\x1b[0m{}", word, split.1)?;
                        } else {
                            write!(f, "\x1b[91m{word}\x1b[0m")?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Changeset;
    use std::io::Write;
    use std::thread;
    use std::time;

    /// convert slice to vector for `assert_eq`
    fn vb(b: &'static [u8]) -> Vec<u8> {
        b.to_vec()
    }

    /// if the format changes, you can use this to help create the test for color
    /// just pass it in and copy-paste (validating that it looks right first of course...)
    #[allow(dead_code)]
    fn debug_bytes(result: &[u8], expected: &[u8]) {
        // sleep for a bit so stderr passes us
        // Static cast
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        thread::sleep(time::Duration::new(0, 2e8 as u32));
        println!("Debug Result:");
        for b in result {
            print!("{}", *b as char);
        }
        println!("Repr Result:");
        repr_bytes(result);
        println!();
        println!("--Result Repr DONE");

        println!("Debug Expected:");
        for b in expected {
            print!("{}", *b as char);
        }
        println!("Repr Expected:");
        repr_bytes(expected);
        println!();
        println!("--Expected Repr DONE");
    }

    /// for helping debugging what the actual bytes are
    /// for writing user tests
    fn repr_bytes(bytes: &[u8]) {
        for b in bytes {
            match *b {
                // 9 => print!("{}", *b as char), // TAB
                b'\n' => print!("\\n"),
                b'\r' => print!("\\r"),
                32..=126 => print!("{}", *b as char), // visible ASCII
                _ => print!(r"\x{b:0>2x}"),
            }
        }
    }

    #[test]
    fn test_display() {
        let text1 = "Roses are red, violets are blue,\n\
                     I wrote this library,\n\
                     just for you.\n\
                     (It's true).";

        let text2 = "Roses are red, violets are blue,\n\
                     I wrote this documentation,\n\
                     just for you.\n\
                     (It's quite true).";
        let expected = b"Roses are red, violets are blue,\n\x1b[91mI wrote this library,\x1b\
            [0m\n\x1b[92mI wrote this documentation,\x1b[0m\njust for you.\n\x1b\
            [91m(It's true).\x1b[0m\n\x1b[92m(It's quite true).\x1b[0m\n";

        let ch = Changeset::new(text1, text2, "\n");
        let mut result: Vec<u8> = Vec::new();
        write!(result, "{ch}").unwrap();
        debug_bytes(&result, expected);
        assert_eq!(result, vb(expected));
    }

    #[test]
    fn test_display_multi() {
        let text1 = "https://localhost:8080/path?query=value";
        let text2 = "https://myapi.com/api/path?query=asset";
        let expected = b"https://\x1b[91mlocalhost:8080\x1b[0m/\x1b[92mmyapi.com\x1b[0m/\x1b[92mapi\x1b[0m/path?query=\x1b[91mvalue\x1b[0m\x1b[92masset\x1b[0m";

        let cg = Changeset::new_multi(text1, text2, &["://", "/", "?", "="]);
        let mut result: Vec<u8> = Vec::new();
        write!(result, "{cg}").unwrap();
        debug_bytes(&result, expected);
        assert_eq!(result, vb(expected));
    }
}
