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

mod dummy;
mod emoticon;
mod image;
mod link;
mod macros;
mod util;

use html2md::{TagHandlerFactory, parse_html_custom};
use quick_xml::{errors::Result, events::Event, reader::Reader, writer::Writer};
use std::collections::HashMap;
use std::io::{BufRead, Write};
use util::JiraServerMap;
pub use util::{ConfluencePageId, ConfluenceServer, JiraServer};

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
    confluence_server: Option<ConfluenceServer>,
    default_space_key: Option<String>,
    default_page_id: Option<ConfluencePageId>,
}

impl ParseOptions {
    pub fn with_jira_server(mut self, server_id: String, jira_server: JiraServer) -> ParseOptions {
        self.jira_server_map.insert(server_id, jira_server);
        self
    }

    pub fn with_confluence_server(mut self, confluence_server: ConfluenceServer) -> ParseOptions {
        self.confluence_server = Some(confluence_server);
        self
    }

    pub fn with_default_page_id(mut self, default_page_id: ConfluencePageId) -> ParseOptions {
        self.default_page_id = Some(default_page_id);
        self
    }

    pub fn with_default_space_key(mut self, default_space_key: String) -> ParseOptions {
        self.default_space_key = Some(default_space_key);
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
            options
                .confluence_server
                .clone()
                .zip(options.default_page_id.clone()),
        )),
    );
    handlers.insert(
        String::from("ac:link"),
        Box::new(link::LinkHandlerFactory::with_url_builder(
            link::LinkHandlerUrlBuilder::new(
                options.confluence_server.clone(),
                options.default_space_key.clone(),
                options.default_page_id.clone(),
            ),
        )),
    );

    let mut reader = Reader::from_str(source.as_ref());
    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);
    remove_cdata(&mut reader, &mut writer).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    parse_html_custom(&text, &handlers)
}
