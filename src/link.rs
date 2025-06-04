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

    fn url_from_page_space_and_title<S: AsRef<str>, T: AsRef<str>>(
        &self,
        space_key: S,
        page_title: T,
    ) -> String {
        let server = self.server.as_ref().expect("missing server");
        server.page_url_with_space_and_title(space_key.as_ref(), page_title.as_ref())
    }

    fn url_from_page_title<S: AsRef<str>>(&self, page_title: S) -> String {
        let default_space = self.default_space.as_deref().expect("missing default space");
        self.url_from_page_space_and_title(default_space, page_title.as_ref())
    }

    fn url_from_attachment_filename<S: AsRef<str>>(&self, filename: S) -> String {
        let server = self.server.as_ref().expect("missing server");
        let page_id = self.page_id.as_ref().expect("missing page id");
        server.attachment_url(page_id, filename.as_ref())
    }

    fn url_from_user_name<S: AsRef<str>>(&self, username: S) -> String {
        let server = self.server.as_ref().expect("missing server");
        server.user_url_with_name(username.as_ref())
    }

    fn url_from_user_key<S: AsRef<str>>(&self, userkey: S) -> String {
        let server = self.server.as_ref().expect("missing server");
        server.user_url_with_key(userkey.as_ref())
    }
}

pub struct LinkHandler {
    start_pos: usize,
    url: Option<String>,
    url_builder: LinkHandlerUrlBuilder,
}

impl LinkHandler {
    pub fn with_url_builder(url_builder: LinkHandlerUrlBuilder) -> Self {
        Self {
            start_pos: 0,
            url: None,
            url_builder,
        }
    }
}

impl TagHandler for LinkHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

        let anchor = get_tag_attr(tag, "ac:anchor");

        let mut page_title = None;
        let mut space_key = None;
        let mut attachment_filename = None;
        let mut user_name = None;
        let mut user_key = None;

        for child in tag.children.borrow().iter() {
            match get_tag_name(child).as_deref() {
                Some("ri:page") => {
                    page_title = get_tag_attr(child, "ri:content-title");
                    space_key = get_tag_attr(child, "ri:space-key");
                }
                Some("ri:space") => {
                    space_key = get_tag_attr(child, "ri:space-key");
                }
                Some("ri:attachment") => {
                    attachment_filename = get_tag_attr(child, "ri:filename");
                }
                Some("ri:user") => {
                    user_name = get_tag_attr(child, "ri:username");
                    user_key = get_tag_attr(child, "ri:userkey");
                }
                _ => (),
            }
        }

        let url = if let Some((space, title)) = space_key.as_deref().zip(page_title.as_deref()) {
            self.url_builder.url_from_page_space_and_title(space, title)
        } else if let Some(title) = page_title {
            self.url_builder.url_from_page_title(title)
        } else if let Some(filename) = attachment_filename {
            self.url_builder.url_from_attachment_filename(filename)
        } else if let Some(name) = user_name {
            self.url_builder.url_from_user_name(name)
        } else if let Some(key) = user_key {
            self.url_builder.url_from_user_key(key)
        } else {
            String::new()
        };

        self.url = if let Some(anchor_name) = anchor {
            format!("{url}#{anchor_name}").into()
        } else {
            url.into()
        };
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        let url = self.url.as_deref().unwrap();
        let index = printer.data.len();
        printer.insert_str(index, &format!("]({url})"));
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
            "[Link to another Confluence Page](https://example.com/confluence/display/CONFL/Page%20Title)"
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
            "[Anchor Link](https://example.com/confluence/display/CONFL/pagetitle#anchor)"
        );
    }

    #[test]
    fn test_link_user_name() {
        markdown_assert_eq!(
            r#"
<ac:link>
<ri:user ri:username="someuser"/>
<ac:plain-text-link-body>User Link</ac:plain-text-link-body>
</ac:link>
"#,
            "[User Link](https://example.com/confluence/users/viewuserprofile.action?username=someuser)"
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
            "[**Link to another Confluence Page**](https://example.com/confluence/display/CONFL/Page%20Title)"
        );
    }
}
