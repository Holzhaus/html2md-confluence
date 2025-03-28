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
