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

mod expand;
mod info;
mod jira;
mod status;

use crate::util::{JiraServerMap, get_tag_name};
use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory, common::get_tag_attr};

#[derive(Default)]
pub struct StructuredMacroHandler {
    macro_specific_handler: Option<Box<dyn TagHandler>>,
    jira_server_map: JiraServerMap,
}

impl StructuredMacroHandler {
    pub fn with_jira_server_map(jira_server_map: JiraServerMap) -> Self {
        Self {
            jira_server_map,
            macro_specific_handler: Default::default(),
        }
    }
}

impl TagHandler for StructuredMacroHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        debug_assert_eq!(get_tag_name(tag).unwrap(), "ac:structured-macro");

        self.macro_specific_handler = match get_tag_attr(tag, "ac:name").as_deref() {
            Some("info") => Some(Box::new(info::InfoMacroHandler::new())),
            Some("tip") => Some(Box::new(info::InfoMacroHandler::new())),
            Some("note") => Some(Box::new(info::InfoMacroHandler::new())),
            Some("warning") => Some(Box::new(info::InfoMacroHandler::new())),
            Some("jira") => Some(Box::new(jira::JiraMacroHandler::with_servers(
                self.jira_server_map.clone(),
            ))),
            Some("expand") => Some(Box::new(expand::ExpandMacroHandler::new())),
            Some("status") => Some(Box::new(status::StatusMacroHandler::new())),
            _ => None,
        };

        if let Some(handler) = self.macro_specific_handler.as_deref_mut() {
            handler.handle(tag, printer);
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        if let Some(handler) = self.macro_specific_handler.as_deref_mut() {
            handler.after_handle(printer);
        };
    }

    fn skip_descendants(&self) -> bool {
        self.macro_specific_handler
            .as_deref()
            .is_some_and(|handler| handler.skip_descendants())
    }
}

pub struct StructuredMacroHandlerFactory {
    jira_server_map: JiraServerMap,
}

impl StructuredMacroHandlerFactory {
    pub fn with_jira_server_map(jira_server_map: JiraServerMap) -> Self {
        Self { jira_server_map }
    }
}

impl TagHandlerFactory for StructuredMacroHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(StructuredMacroHandler::with_jira_server_map(
            self.jira_server_map.clone(),
        ))
    }
}
