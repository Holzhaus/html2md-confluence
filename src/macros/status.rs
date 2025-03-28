use crate::util::{get_tag_name, get_text_content};
use html2md::{Handle, StructuredPrinter, TagHandler, common::get_tag_attr};

pub struct StatusMacroHandler;

impl StatusMacroHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl TagHandler for StatusMacroHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let children = tag.children.borrow();
        let title = children
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

        if let Some(title) = title {
            printer.append_str(&title);
        }
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use html2md::{TagHandlerFactory, parse_html_custom};
    use std::collections::HashMap;

    struct StatusMacroHandlerFactory;
    impl TagHandlerFactory for StatusMacroHandlerFactory {
        fn instantiate(&self) -> Box<dyn TagHandler> {
            Box::new(StatusMacroHandler::new())
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
            Box::new(StatusMacroHandlerFactory {}),
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
    fn test() {
        markdown_assert_eq!(
            r#"
 <ac:structured-macro ac:name="status">
    <ac:parameter ac:name="colour">Green</ac:parameter>
    <ac:parameter ac:name="title">On track</ac:parameter>
    <ac:parameter ac:name="subtle">true</ac:parameter>
</ac:structured-macro>
"#,
            "On track"
        );
    }
}
