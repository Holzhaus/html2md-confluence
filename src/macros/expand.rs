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

pub struct ExpandMacroHandler {
    start_pos: usize,
    title: Option<String>,
}

impl ExpandMacroHandler {
    pub fn new() -> Self {
        Self {
            start_pos: 0,
            title: None,
        }
    }
}

impl TagHandler for ExpandMacroHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

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
        printer.append_str(&format!(
            "<details><summary>{title}</summary>",
            title = self.title.as_deref().unwrap_or("Click here to expand...")
        ));
        printer.insert_newline();
        printer.insert_newline();
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.append_str("</details>");
        printer.insert_newline();
        printer.insert_newline();
    }
}

#[cfg(test)]
mod test {
    use crate::markdown_assert_eq;

    #[test]
    fn test_with_title() {
        markdown_assert_eq!(
            r#"
<ac:structured-macro ac:name="expand">
  <ac:parameter ac:name="title">Click Me</ac:parameter>
  <ac:rich-text-body>
    <p>
      <span>This is </span> <em>important</em> <span>information.</span>
    </p>
  </ac:rich-text-body>
</ac:structured-macro>
"#,
            "\
<details><summary>Click Me</summary>

This is *important* information.

</details>"
        );
    }

    #[test]
    fn test_without_title() {
        markdown_assert_eq!(
            r#"
<ac:structured-macro ac:name="expand">
  <ac:rich-text-body>
    <p>
      <span>This is </span> <em>important</em> <span>information.</span>
    </p>
  </ac:rich-text-body>
</ac:structured-macro>
"#,
            "\
<details><summary>Click here to expand...</summary>

This is *important* information.

</details>"
        );
    }
}
