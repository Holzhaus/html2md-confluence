// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If
// not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::util::{get_tag_name, get_text_content};
use html2md::{Handle, StructuredPrinter, TagHandler, common::get_tag_attr};
use std::str::FromStr;

enum HighlightType {
    Info,
    Tip,
    Note,
    Warning,
}

impl HighlightType {
    const GFM_ALERT_NOTE: &str = "[!NOTE]";
    const GFM_ALERT_TIP: &str = "[!TIP]";
    const GFM_ALERT_INFO: &str = "[!IMPORTANT]";
    const GFM_ALERT_WARNING: &str = "[!WARNING]";

    fn as_gfm_alert(&self) -> &'static str {
        match self {
            Self::Info => Self::GFM_ALERT_INFO,
            Self::Tip => Self::GFM_ALERT_TIP,
            Self::Note => Self::GFM_ALERT_NOTE,
            Self::Warning => Self::GFM_ALERT_WARNING,
        }
    }
}

impl FromStr for HighlightType {
    type Err = &'static str;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "info" => Ok(Self::Info),
            "tip" => Ok(Self::Tip),
            "note" => Ok(Self::Note),
            "warning" => Ok(Self::Warning),
            _ => Err("invalid name"),
        }
    }
}

pub struct InfoMacroHandler {
    start_pos: usize,
    highlight_type: HighlightType,
    title: Option<String>,
}

impl InfoMacroHandler {
    const QUOTE: &str = "> ";

    pub fn new() -> Self {
        Self {
            start_pos: 0,
            title: None,
            highlight_type: HighlightType::Info,
        }
    }
}

impl TagHandler for InfoMacroHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

        self.highlight_type = get_tag_attr(tag, "ac:name")
            .as_deref()
            .and_then(|s| HighlightType::from_str(s).ok())
            .expect("invalid name");
        let children = tag.children.borrow();
        self.title = children
            .iter()
            .filter_map(|child| {
                if get_tag_name(child).is_some_and(|name| name == "ac:parameter") {
                    get_tag_attr(child, "ac:name").map(|value| (value, child))
                } else {
                    None
                }
            })
            .find_map(|(param_name, param)| {
                if &param_name == "title" {
                    let s = get_text_content(param);
                    if s.is_empty() { None } else { Some(s) }
                } else {
                    None
                }
            });

        printer.insert_newline();
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        let mut index = printer.data.len();
        while index > self.start_pos {
            if printer.data.as_bytes().get(index).copied() == Some(b'\n') {
                printer.insert_str(index + 1, Self::QUOTE);
            }
            index -= 1;
        }

        let mut pos = self.start_pos + 1;
        printer.insert_str(pos, Self::QUOTE);
        pos += Self::QUOTE.len();
        let gfm_alert = self.highlight_type.as_gfm_alert();
        printer.insert_str(pos, gfm_alert);
        pos += gfm_alert.len();
        if let Some(title) = self.title.as_deref() {
            printer.insert_str(
                pos,
                format!("\n{quote}**{title}**", quote = Self::QUOTE).as_str(),
            );
        }

        printer.insert_newline();
        printer.insert_newline();
    }
}

#[cfg(test)]
mod test {
    use crate::markdown_assert_eq;

    #[test]
    fn test_info_with_title() {
        markdown_assert_eq!(
            r#"
<ac:structured-macro ac:name="info">
  <ac:parameter ac:name="icon">false</ac:parameter>
  <ac:parameter ac:name="title">Some info</ac:parameter>
  <ac:rich-text-body>
    <p>
      <span>This is </span> <em>important</em> <span>information.</span>
    </p>
  </ac:rich-text-body>
</ac:structured-macro>
"#,
            "\
> [!IMPORTANT]
> **Some info**
>
> This is *important* information.
>
>"
        );
    }

    #[test]
    fn test_note_without_title() {
        markdown_assert_eq!(
            r#"
<ac:structured-macro ac:name="note">
  <ac:parameter ac:name="icon">true</ac:parameter>
  <ac:rich-text-body>
<p>
      <span>This is </span> <em>important</em> <span>information.</span>
    </p>
  </ac:rich-text-body>
</ac:structured-macro>
"#,
            "\
> [!NOTE]
>
> This is *important* information.
>
>"
        );
    }
}
