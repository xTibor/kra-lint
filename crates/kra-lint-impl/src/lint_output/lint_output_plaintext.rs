#![cfg(feature = "output-plaintext")]

use std::io::Write;

use itertools::Itertools;
use unicode_width::UnicodeWidthStr;

use crate::lint_output::{LintMessagesCollection, LintMessagesCollectionEntry, LintMessagesEntry, LintOutputError};

pub(crate) fn to_writer<W>(writer: &mut W, message_collection: &LintMessagesCollection) -> Result<(), LintOutputError>
where
    W: Write,
{
    for LintMessagesCollectionEntry { path, messages } in message_collection {
        for (message_title, group) in
            &messages.into_iter().group_by(|LintMessagesEntry { message_title, .. }| message_title)
        {
            let indent_size = path.to_string().width();
            let indent_str = format!("{}  | ", " ".repeat(indent_size));

            writer.write_all(format!("{}: {}\n", path, message_title).as_bytes())?;
            for LintMessagesEntry { message_metadata, .. } in group {
                writer.write_all(format!("{}{}\n", indent_str, message_metadata.iter().join(", ")).as_bytes())?;
            }
            writer.write_all(b"\n")?;
        }
    }

    match message_collection.message_count() {
        0 => writer.write_all("kra-lint: No issues found\n".as_bytes())?,
        1 => writer.write_all("kra-lint: One issue found\n".as_bytes())?,
        n => writer.write_all(format!("kra-lint: {} issues found\n", n).as_bytes())?,
    }

    Ok(())
}
