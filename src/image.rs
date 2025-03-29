use crate::util::{ConfluencePageId, ConfluenceServer, get_tag_name};
use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory, common::get_tag_attr};

#[derive(Default)]
pub struct ImageHandler {
    page: Option<(ConfluenceServer, ConfluencePageId)>,
}

impl ImageHandler {
    pub fn with_confluence_page(page: Option<(ConfluenceServer, ConfluencePageId)>) -> Self {
        Self { page }
    }
}

impl TagHandler for ImageHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let caption = get_tag_attr(tag, "ac:title").or_else(|| get_tag_attr(tag, "ac:alt"));

        let children = tag.children.borrow();
        let Some(url) = children.iter().find_map(|child| {
            get_tag_name(child).as_deref().and_then(|name| match name {
                "ri:url" => get_tag_attr(child, "ri:value"),
                "ri:attachment" => self.page.as_ref().and_then(|(server, page_id)| {
                    get_tag_attr(child, "ri:filename")
                        .map(|filename| server.attachment_url(page_id, filename))
                }),
                _ => None,
            })
        }) else {
            return;
        };

        let title = caption.as_deref().unwrap_or("");
        printer.append_str(&format!("![{title}]({url})"));
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}

    fn skip_descendants(&self) -> bool {
        true
    }
}
pub struct ImageHandlerFactory {
    page: Option<(ConfluenceServer, ConfluencePageId)>,
}

impl ImageHandlerFactory {
    pub fn with_confluence_page(page: Option<(ConfluenceServer, ConfluencePageId)>) -> Self {
        Self { page }
    }
}

impl TagHandlerFactory for ImageHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(ImageHandler::with_confluence_page(self.page.clone()))
    }
}
