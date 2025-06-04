use crate::util::{ConfluencePageId, ConfluenceServer, get_tag_name};
use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory, common::get_tag_attr};

#[derive(Debug, Clone)]
pub struct LinkHandlerUrlBuilder {
    server: Option<ConfluenceServer>,
    default_space: Option<String>,
    page_id: Option<ConfluencePageId>,
}

impl LinkHandlerUrlBuilder {
    pub fn new(
        server: Option<ConfluenceServer>,
        default_space: Option<String>,
        page_id: Option<ConfluencePageId>,
    ) -> Self {
        Self {
            server,
            default_space,
            page_id,
        }
    }

    fn url_from_page_title<S: AsRef<str>>(&self, page_title: S) -> Option<String> {
        self.server
            .as_ref()
            .zip(self.default_space.as_deref())
            .map(|(server, default_space)| {
                server.page_url_with_space_and_title(default_space, page_title.as_ref())
            })
    }

    fn url_from_attachment_filename<S: AsRef<str>>(&self, filename: S) -> Option<String> {
        self.server
            .as_ref()
            .zip(self.page_id.as_ref())
            .map(|(server, page_id)| server.attachment_url(page_id, filename.as_ref()))
    }
}

pub struct LinkHandler {
    start_pos: usize,
    anchor: Option<String>,
    url: Option<String>,
    url_builder: LinkHandlerUrlBuilder,
}

impl LinkHandler {
    pub fn with_url_builder(url_builder: LinkHandlerUrlBuilder) -> Self {
        Self {
            start_pos: 0,
            anchor: None,
            url: None,
            url_builder,
        }
    }
}

impl LinkHandler {
    fn get_url_from_ri_page(&self, tag: &Handle) -> Option<String> {
        get_tag_attr(tag, "ri:content-title")
            .and_then(|page_name| self.url_builder.url_from_page_title(page_name))
    }

    fn get_url_from_ri_attachment(&self, tag: &Handle) -> Option<String> {
        get_tag_attr(tag, "ri:filename")
            .and_then(|filename| self.url_builder.url_from_attachment_filename(filename))
    }
}

impl TagHandler for LinkHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

        self.anchor = get_tag_attr(tag, "ac:anchor");

        let children = tag.children.borrow();
        self.url = children
            .iter()
            .filter_map(|child| {
                match get_tag_name(child).as_deref() {
                    Some("ri:page") => Some(self.get_url_from_ri_page(child).unwrap()),
                    Some("ri:attachment") => Some(self.get_url_from_ri_attachment(child).unwrap()),
                    _ => None,
                }
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
            })
            .next();
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        let url = self.url.as_deref().unwrap_or("");
        let index = printer.data.len();
        let markdown = if let Some(anchor) = self.anchor.as_deref() {
            format!("]({url}#{anchor})")
        } else {
            format!("]({url})")
        };
        printer.insert_str(index, markdown.as_str());
        printer.insert_str(self.start_pos, "[");
    }
}

pub struct LinkHandlerFactory {
    url_builder: LinkHandlerUrlBuilder,
}

impl LinkHandlerFactory {
    pub fn with_url_builder(url_builder: LinkHandlerUrlBuilder) -> Self {
        Self { url_builder }
    }
}

impl TagHandlerFactory for LinkHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(LinkHandler::with_url_builder(self.url_builder.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use html2md::parse_html_custom;
    use std::collections::HashMap;
    use std::str::FromStr;

    fn get_handlers() -> HashMap<String, Box<(dyn TagHandlerFactory + 'static)>> {
        let mut handlers: HashMap<_, Box<(dyn TagHandlerFactory + 'static)>> = HashMap::new();
        handlers.insert(
            String::from("ac:link"),
            Box::new(LinkHandlerFactory::with_url_builder(
                LinkHandlerUrlBuilder::new(
                    ConfluenceServer::from_str("https://example.com/confluence").ok(),
                    Some("CONFL".to_string()),
                    Some(ConfluencePageId::from(1337)),
                ),
            )),
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
    fn test_link_ri_page() {
        markdown_assert_eq!(
            r#"
<ac:link>
<ri:page ri:content-title="Page Title" />
<ac:plain-text-link-body>Link to another Confluence Page</ac:plain-text-link-body>
</ac:link>
"#,
            "[Link to another Confluence Page](https://example.com/confluence/CONFL/Page%20Title)"
        );
    }

    #[test]
    fn test_link_ri_attachment() {
        markdown_assert_eq!(
            r#"
<ac:link>
<ri:attachment ri:filename="atlassian_logo.gif" />
<ac:plain-text-link-body>Link to a Confluence Attachment</ac:plain-text-link-body>
</ac:link>
"#,
            "[Link to a Confluence Attachment](https://example.com/confluence/download/attachments/1337/atlassian_logo.gif)"
        );
    }

    #[test]
    fn test_link_anchor_same_page() {
        markdown_assert_eq!(
            r#"
<ac:link ac:anchor="anchor">
<ac:plain-text-link-body>Anchor Link</ac:plain-text-link-body>
</ac:link>
"#,
            "[Anchor Link](#anchor)"
        );
    }

    #[test]
    fn test_link_anchor_another_page() {
        markdown_assert_eq!(
            r#"
<ac:link ac:anchor="anchor">
<ri:page ri:content-title="pagetitle"/>
<ac:plain-text-link-body>Anchor Link</ac:plain-text-link-body>
</ac:link>
"#,
            "[Anchor Link](https://example.com/confluence/CONFL/pagetitle#anchor)"
        );
    }

    #[test]
    fn test_link_ri_page_with_rich_text_body() {
        markdown_assert_eq!(
            r#"
<ac:link>
<ri:page ri:content-title="Page Title" />
<ac:link-body><b>Link to another Confluence Page<b></ac:link-body>
</ac:link>
"#,
            "[**Link to another Confluence Page**](https://example.com/confluence/CONFL/Page%20Title)"
        );
    }
}
