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
    use crate::{ParseOptions, parse_confluence};

    macro_rules! markdown_assert_eq {
        ($html:expr, $markdown:expr) => {
            let options = ParseOptions::default();
            let md = parse_confluence($html, &options);
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
