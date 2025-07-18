//! Functions to find the difference between two texts (strings).
//!
//! Usage
//! ----------
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! difference_rs = "3.0"
//! ```
//!
//! Now you can use the crate in your code
//!
//!
//! ## Examples
//!
//! See [Examples.md](Examples.md) for more examples.
//!
//! ```rust
//! use difference_rs::{Difference, Changeset};
//!
//! let changeset = Changeset::new("test", "tent", "");
//!
//! assert_eq!(changeset.diffs, vec![
//!   Difference::Same("te".to_string()),
//!   Difference::Rem("s".to_string()),
//!   Difference::Add("n".to_string()),
//!   Difference::Same("t".to_string())
//! ]);
//! ```

#![crate_name = "difference_rs"]
#![doc(html_root_url = "http://docs.rs/difference-rs")]
#![deny(missing_docs)]
#![deny(warnings)]

mod display;
mod lcs;
mod merge;
mod multi;

use std::char::REPLACEMENT_CHARACTER;

use crate::lcs::lcs;
use crate::merge::merge;

/// Defines the contents of a changeset
/// Changesets will be delivered in order of appearance in the original string
/// Sequences of the same kind will be grouped into one Difference
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Difference {
    /// Sequences that are the same
    Same(String),
    /// Sequences that are an addition (don't appear in the first string)
    Add(String),
    /// Sequences that are a removal (don't appear in the second string)
    Rem(String),
}

/// The information about a full changeset
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Changeset {
    /// An ordered vector of `Difference` objects, corresponding
    /// to the differences within the text
    pub diffs: Vec<Difference>,
    /// The split used when creating the `Changeset`
    /// Common splits are `""` for char-level, `" "` for word-level and `"\n"` for line-level.
    pub split: String,
    /// The edit distance of the `Changeset`
    pub distance: i128,
}

/// The information about a full changeset when regarding a multi split changeset
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangesetMulti {
    /// An ordered vector of `Difference` objects, corresponding
    /// to the differences within the text
    pub diffs: Vec<Difference>,
    /// The splits used when creating the `Changeset` with their respective indexes in the origin string.
    pub splits: Vec<(usize, String)>,
    /// The splits used when creating the `Changeset` with their respective indexes in the edit string.
    pub edit_splits: Vec<(usize, String)>,
    /// The edit distance of the `Changeset`
    pub distance: i128,
}

impl Changeset {
    /// Calculates the edit distance and the changeset for two given strings.
    /// The first string is assumed to be the "original", the second to be an
    /// edited version of the first. The third parameter specifies how to split
    /// the input strings, leading to a more or less exact comparison.
    ///
    /// Common splits are `""` for char-level, `" "` for word-level and `"\n"` for line-level.
    ///
    /// Outputs the edit distance (how much the two strings differ) and a "changeset", that is
    /// a `Vec` containing `Difference`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use difference_rs::{Changeset, Difference};
    ///
    /// let changeset = Changeset::new("test", "tent", "");
    ///
    /// assert_eq!(changeset.diffs, vec![
    ///     Difference::Same("te".to_string()),
    ///     Difference::Rem("s".to_string()),
    ///     Difference::Add("n".to_string()),
    ///     Difference::Same("t".to_string())
    /// ]);
    /// ```
    #[must_use]
    pub fn new(orig: &str, edit: &str, split: &str) -> Changeset {
        let (dist, common) = lcs(orig, edit, split);
        Changeset {
            diffs: merge(orig, edit, &common, split),
            split: split.to_string(),
            distance: dist,
        }
    }

    /// Creates a `Changeset` with multiple possible splits.
    /// The first string is assumed to be the "original", the second to be an
    /// edited version of the first. The third parameter specifies how to split
    /// the input strings, leading to a more or less exact comparison.
    ///
    /// Outputs the edit distance (how much the two strings differ), original string splits, edit string splits, a "changeset", that is
    /// a `Vec` containing `Difference`s.
    ///
    /// Obs: Splits are included inside the `Difference` vector, as it is the only way to correctly rebuild strings, which differs from
    /// `Changeset::new` that all spaces are filled by a single split.
    ///
    /// # Examples
    ///
    /// ```
    /// use difference_rs::{Changeset, Difference};
    ///
    /// let changeset = Changeset::new_multi(
    ///    "https://localhost:8080/path?query=value",
    ///    "https://myapi.com/api/path?query=asset",
    ///    &["://", "/", "?", "="],
    /// );
    ///
    /// assert_eq!(changeset.diffs, vec![
    ///     Difference::Same("https://".to_string()),
    ///     Difference::Rem("localhost:8080/".to_string()),
    ///     Difference::Add("myapi.com/api/".to_string()),
    ///     Difference::Same("path?query=".to_string()),
    ///     Difference::Rem("value".to_string()),
    ///     Difference::Add("asset".to_string()),
    /// ]);
    /// ```
    #[must_use]
    pub fn new_multi(orig: &str, edit: &str, splits: &[&str]) -> ChangesetMulti {
        let matched_splits = splits
            .iter()
            .flat_map(|split| orig.match_indices(*split))
            .map(|(k, v)| (k, v.to_string()))
            .collect::<Vec<(usize, String)>>();
        let edit_splits = splits
            .iter()
            .flat_map(|split| edit.match_indices(*split))
            .map(|(k, v)| (k, v.to_string()))
            .collect::<Vec<(usize, String)>>();

        let mut aux_orig = orig.to_string();
        let mut aux_edit = edit.to_string();
        let replacement = REPLACEMENT_CHARACTER.to_string();
        for split in splits {
            aux_orig = aux_orig.replace(split, &replacement);
            aux_edit = aux_edit.replace(split, &replacement);
        }

        let changeset = Changeset::new(&aux_orig, &aux_edit, &replacement);
        ChangesetMulti::from((changeset, matched_splits, edit_splits))
    }
}

/// Assert the difference between two strings. Works like diff, but takes
/// a fourth parameter that is the expected edit distance (e.g. 0 if you want to
/// test for equality).
///
/// Remember that edit distance might not be equal to your understanding of difference,
/// for example the words "Rust" and "Dust" have an edit distance of 2 because two changes (a
/// removal and an addition) are required to make them look the same.
///
/// Will print an error with a colorful diff in case of failure.
#[macro_export]
macro_rules! assert_diff {
    ($orig:expr_2021 , $edit:expr_2021, $split: expr_2021, $expected: expr_2021) => {{
        let orig = $orig;
        let edit = $edit;

        let changeset = $crate::Changeset::new(orig, edit, &($split));
        if changeset.distance != $expected {
            println!("{}", changeset);
            panic!(
                "assertion failed: edit distance between {:?} and {:?} is {} and not {}, see \
                    diffset above",
                orig,
                edit,
                changeset.distance,
                &($expected)
            )
        }
    }};
}

#[test]
fn test_diff() {
    let text1 = "Roses are red, violets are blue,\n\
                 I wrote this library,\n\
                 just for you.\n\
                 (It's true).";

    let text2 = "Roses are red, violets are blue,\n\
                 I wrote this documentation,\n\
                 just for you.\n\
                 (It's quite true).";

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(changeset.distance, 4);

    assert_eq!(
        changeset.diffs,
        vec![
            Difference::Same("Roses are red, violets are blue,".to_string()),
            Difference::Rem("I wrote this library,".to_string()),
            Difference::Add("I wrote this documentation,".to_string()),
            Difference::Same("just for you.".to_string()),
            Difference::Rem("(It's true).".to_string()),
            Difference::Add("(It's quite true).".to_string()),
        ]
    );
}

#[test]
fn test_diff_brief() {
    let text1 = "Hello\nworld";
    let text2 = "Ola\nmundo";

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(
        changeset.diffs,
        vec![
            Difference::Rem("Hello\nworld".to_string()),
            Difference::Add("Ola\nmundo".to_string()),
        ]
    );
}

#[test]
#[cfg(feature = "serde")]
fn test_diff_smaller_line_count_on_left() {
    let text1 = "Hello\nworld";
    let text2 = "Ola\nworld\nHow is it\ngoing?";

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(
        changeset.diffs,
        vec![
            Difference::Rem("Hello".to_string()),
            Difference::Add("Ola".to_string()),
            Difference::Same("world".to_string()),
            Difference::Add("How is it\ngoing?".to_string()),
        ]
    );

    let json = serde_json::to_string(&changeset).unwrap();

    assert_eq!(
        json,
        r#"{"diffs":[{"Rem":"Hello"},{"Add":"Ola"},{"Same":"world"},{"Add":"How is it\ngoing?"}],"split":"\n","distance":4}"#
    );
}

#[test]
fn test_diff_smaller_line_count_on_right() {
    let text1 = "Hello\nworld\nWhat a \nbeautiful\nday!";
    let text2 = "Ola\nworld";

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(
        changeset.diffs,
        vec![
            Difference::Rem("Hello".to_string()),
            Difference::Add("Ola".to_string()),
            Difference::Same("world".to_string()),
            Difference::Rem("What a \nbeautiful\nday!".to_string()),
        ]
    );
}

#[test]
fn test_diff_similar_text_with_smaller_line_count_on_right() {
    let text1 = "Hello\nworld\nWhat a \nbeautiful\nday!";
    let text2 = "Hello\nwoRLd";

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(
        changeset.diffs,
        vec![
            Difference::Same("Hello".to_string()),
            Difference::Rem("world\nWhat a \nbeautiful\nday!".to_string()),
            Difference::Add("woRLd".to_string()),
        ]
    );
}

#[test]
fn test_diff_similar_text_with_similar_line_count() {
    let text1 = "Hello\nworld\nWhat a \nbeautiful\nday!";
    let text2 = "Hello\nwoRLd\nbeautiful";

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(
        changeset.diffs,
        vec![
            Difference::Same("Hello".to_string()),
            Difference::Rem("world\nWhat a ".to_string()),
            Difference::Add("woRLd".to_string()),
            Difference::Same("beautiful".to_string()),
            Difference::Rem("day!".to_string()),
        ]
    );
}

#[test]
#[should_panic = r#"assertion failed: edit distance between "Roses are red, violets are blue,\nI wrote this library,\njust for you.\n(It's true)." and "Roses are red, violets are blue,\nI wrote this documentation,\njust for you.\n(It's quite true)." is 2 and not 0, see diffset above"#]
fn test_assert_diff_panic() {
    let text1 = "Roses are red, violets are blue,\n\
                 I wrote this library,\n\
                 just for you.\n\
                 (It's true).";

    let text2 = "Roses are red, violets are blue,\n\
                 I wrote this documentation,\n\
                 just for you.\n\
                 (It's quite true).";

    assert_diff!(text1, text2, "\n'", 0);
}

#[test]
fn test_assert_diff() {
    let text1 = "Roses are red, violets are blue";

    let text2 = "Roses are green, violets are blue";

    assert_diff!(text1, text2, " ", 2);
}

#[test]
fn test_multi_pattern() {
    let cg = Changeset::new_multi("hello,world now", "hellow,world later", &[",", " "]);
    let expected = ChangesetMulti {
        diffs: vec![
            Difference::Rem("hello,".to_string()),
            Difference::Add("hellow,".to_string()),
            Difference::Same("world ".to_string()),
            Difference::Rem("now".to_string()),
            Difference::Add("later".to_string()),
        ],
        splits: vec![(5, ",".to_string()), (11, " ".to_string())],
        edit_splits: vec![(6, ",".to_string()), (12, " ".to_string())],
        distance: 4,
    };

    assert_eq!(cg, expected);
}

#[test]
fn test_multi_uri_pattern() {
    let cg = Changeset::new_multi(
        "https://localhost:8080/path?query=value",
        "https://myapi.com/api/path?query=asset",
        &["://", "/", "?", "="],
    );
    let expected = ChangesetMulti {
        diffs: vec![
            Difference::Same("https://".to_string()),
            Difference::Rem("localhost:8080/".to_string()),
            Difference::Add("myapi.com/api/".to_string()),
            Difference::Same("path?query=".to_string()),
            Difference::Rem("value".to_string()),
            Difference::Add("asset".to_string()),
        ],
        splits: vec![
            (5, "://".to_string()),
            (6, "/".to_string()),
            (7, "/".to_string()),
            (22, "/".to_string()),
            (27, "?".to_string()),
            (33, "=".to_string()),
        ],
        edit_splits: vec![
            (5, "://".to_string()),
            (6, "/".to_string()),
            (7, "/".to_string()),
            (17, "/".to_string()),
            (21, "/".to_string()),
            (26, "?".to_string()),
            (32, "=".to_string()),
        ],
        distance: 5,
    };

    assert_eq!(cg, expected);
}
