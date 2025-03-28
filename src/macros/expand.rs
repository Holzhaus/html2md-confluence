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
    use super::*;
    use html2md::{TagHandlerFactory, parse_html_custom};
    use std::collections::HashMap;

    struct ExpandMacroHandlerFactory;
    impl TagHandlerFactory for ExpandMacroHandlerFactory {
        fn instantiate(&self) -> Box<dyn TagHandler> {
            Box::new(ExpandMacroHandler::new())
        }
    }

    struct DummyRecursiveHandler;
    impl TagHandler for DummyRecursiveHandler {
        fn handle(&mut self, _tag: &Handle, _printer: &mut StructuredPrinter) {}
        fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
        fn skip_descendants(&self) -> bool {
            return true;
        }
    }

    struct DummyRecursiveHandlerFactory;
    impl TagHandlerFactory for DummyRecursiveHandlerFactory {
        fn instantiate(&self) -> Box<dyn TagHandler> {
            Box::new(DummyRecursiveHandler {})
        }
    }

    fn get_handlers() -> HashMap<String, Box<(dyn TagHandlerFactory + 'static)>> {
        let mut handlers: HashMap<_, Box<(dyn TagHandlerFactory + 'static)>> = HashMap::new();
        handlers.insert(
            String::from("ac:structured-macro"),
            Box::new(ExpandMacroHandlerFactory {}),
        );
        handlers.insert(
            String::from("ac:parameter"),
            Box::new(DummyRecursiveHandlerFactory {}),
        );
        handlers
    }

    macro_rules! markdown_assert_eq {
        ($html:expr, $markdown:expr) => {
            let handlers = get_handlers();
            let md = parse_html_custom($html, &handlers);
            assert_eq!(md, $markdown);
        };
    }

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
