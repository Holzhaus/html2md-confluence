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

use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory, common::get_tag_attr};

#[derive(Default)]
pub struct EmoticonHandler;

fn confluence_emoticon_to_markdown<S: AsRef<str>>(name: S) -> Option<&'static str> {
    match name.as_ref() {
        "smile" => ":slightly_smiling_face:".into(),    // 🙂
        "sad" => ":slightly_frowning_face:".into(),     // 🙁
        "cheeky" => ":stuck_out_tongue:".into(),        // 😛
        "laugh" => ":smiley:".into(),                   // 😃
        "wink" => ":wink:".into(),                      // 😉
        "thumbs-up" => ":thumbsup:".into(),             // 👍
        "thumbs-down" => ":thumbsdown:".into(),         // 👎
        "information" => ":information_source:".into(), // ℹ️
        "tick" => ":white_check_mark:".into(),          // ✅
        "cross" => ":x:".into(),                        // ❌
        "warning" => ":warning:".into(),                // ⚠️
        "plus" => ":heavy_plus_sign:".into(),           // ➕
        "minus" => ":heavy_minus_sign:".into(),         // ➖
        "question" => ":question:".into(),              // ❓
        "light-on" => ":bulb:".into(),                  // 💡
        "light-off" => ":bulb: (off)".into(),           // 💡
        "yellow-star" => ":star:".into(),               // ⭐
        "red-star" => ":star: (red)".into(),            // ⭐
        "green-star" => ":star: (green)".into(),        // ⭐
        "blue-star" => ":star: (blue)".into(),          // ⭐
        _ => None,
    }
}

impl TagHandler for EmoticonHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let name = get_tag_attr(tag, "ac:name");
        let Some(emoticon) = name.as_deref().and_then(confluence_emoticon_to_markdown) else {
            return;
        };

        printer.append_str(emoticon);
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}

    fn skip_descendants(&self) -> bool {
        true
    }
}

pub struct EmoticonHandlerFactory;

impl TagHandlerFactory for EmoticonHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(EmoticonHandler {})
    }
}

#[cfg(test)]
mod test {
    use crate::markdown_assert_eq;

    #[test]
    fn test_tick() {
        markdown_assert_eq!(r#"<ac:emoticon ac:name="tick"/>"#, ":white_check_mark:");
    }

    #[test]
    fn test_laugh() {
        markdown_assert_eq!(r#"<ac:emoticon ac:name="laugh"/>"#, ":smiley:");
    }
}
