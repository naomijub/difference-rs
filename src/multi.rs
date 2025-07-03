use std::char::REPLACEMENT_CHARACTER;

use crate::{Changeset, ChangesetMulti, Difference};

impl From<(Changeset, Vec<(usize, String)>, Vec<(usize, String)>)> for ChangesetMulti {
    fn from(
        (changeset, orig_splits, edit_splits): (
            Changeset,
            Vec<(usize, String)>,
            Vec<(usize, String)>,
        ),
    ) -> Self {
        let distance = changeset.distance;
        let mut orig_counter = 0;
        let mut edit_counter = 0;
        let diffs = changeset
            .diffs
            .into_iter()
            .map(|d| match d {
                Difference::Same(ref x) => {
                    let orig = x.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                    let mut s = String::new();
                    for word in orig {
                        orig_counter += word.len();
                        edit_counter += word.len();
                        s.push_str(word);
                        if let Some(split) = orig_splits
                            .iter()
                            .find(|(idx, _split)| idx == &orig_counter)
                        {
                            orig_counter += split.1.len();
                            edit_counter += split.1.len();

                            s.push_str(&split.1);
                        }
                    }
                    Difference::Same(s)
                }
                Difference::Add(ref x) => {
                    let edit = x.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                    let mut s = String::new();
                    for word in edit {
                        edit_counter += word.len();
                        s.push_str(word);
                        if let Some(split) = edit_splits
                            .iter()
                            .find(|(idx, _split)| idx == &edit_counter)
                        {
                            edit_counter += split.1.len();
                            s.push_str(&split.1);
                        }
                    }
                    Difference::Add(s)
                }
                Difference::Rem(ref x) => {
                    let orig = x.as_str().split(REPLACEMENT_CHARACTER).collect::<Vec<_>>();
                    let mut s = String::new();
                    for word in orig {
                        orig_counter += word.len();
                        s.push_str(word);
                        if let Some(split) = orig_splits
                            .iter()
                            .find(|(idx, _split)| idx == &orig_counter)
                        {
                            orig_counter += split.1.len();
                            s.push_str(&split.1);
                        }
                    }
                    Difference::Rem(s)
                }
            })
            .collect();
        ChangesetMulti {
            splits: orig_splits,
            edit_splits,
            distance,
            diffs,
        }
    }
}
