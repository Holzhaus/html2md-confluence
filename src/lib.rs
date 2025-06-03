mod dummy;
mod emoticon;
mod image;
mod macros;
mod util;

use html2md::{TagHandlerFactory, parse_html_custom};
use std::collections::HashMap;
use util::{ConfluencePageId, ConfluenceServer, JiraServer, JiraServerMap};
use quick_xml::{
    errors::Result,
events::Event,
reader::Reader,
writer::Writer,
};
use std::io::{BufRead, Write};


fn remove_cdata<R: BufRead, W: Write>(
    reader: &mut Reader<R>,
    writer: &mut Writer<W>,
) -> Result<()> {
    let mut buf = Vec::with_capacity(2048);

    loop {
        let event = match reader.read_event_into(&mut buf)? {
            Event::CData(text) => Event::Text(text.escape()?),
            Event::Eof => break Ok(()),
            other_event => other_event,
        };

        writer.write_event(event.borrow())?;
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParseOptions {
    jira_server_map: JiraServerMap,
    server: Option<ConfluenceServer>,
    page_id: Option<ConfluencePageId>,
}

impl ParseOptions {
    pub fn with_jira_server(mut self, server_id: String, jira_server: JiraServer) -> ParseOptions {
        self.jira_server_map.insert(server_id, jira_server);
        self
    }

    pub fn with_confluence_server(mut self, server: ConfluenceServer) -> ParseOptions {
        self.server = Some(server);
        self
    }

    pub fn with_page_id(mut self, page_id: ConfluencePageId) -> ParseOptions {
        self.page_id = Some(page_id);
        self
    }
}

pub fn parse_confluence<S: AsRef<str>>(source: S, options: &ParseOptions) -> String {
    let mut handlers: HashMap<_, Box<(dyn TagHandlerFactory + 'static)>> = HashMap::new();
    handlers.insert(
        String::from("ac:structured-macro"),
        Box::new(macros::StructuredMacroHandlerFactory::with_jira_server_map(
            options.jira_server_map.clone(),
        )),
    );
    handlers.insert(
        String::from("ac:parameter"),
        Box::new(dummy::RecursiveDummyHandlerFactory {}),
    );
    handlers.insert(
        String::from("ac:emoticon"),
        Box::new(emoticon::EmoticonHandlerFactory {}),
    );
    handlers.insert(
        String::from("ac:image"),
        Box::new(image::ImageHandlerFactory::with_confluence_page(
            options.server.clone().zip(options.page_id.clone()),
        )),
    );

    let mut reader = Reader::from_str(source.as_ref());
    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);
    remove_cdata(&mut reader, &mut writer).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    parse_html_custom(&text, &handlers)
}
