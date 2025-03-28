mod expand;
mod info;
mod jira;
mod status;

use crate::util::get_tag_name;
use html2md::{Handle, StructuredPrinter, TagHandler, common::get_tag_attr};

#[derive(Default)]
pub struct StructuredMacroHandler {
    macro_specific_handler: Option<Box<dyn TagHandler>>,
    jira_server_map: jira::JiraServerMap,
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

#[derive(Default)]
pub struct ParameterHandler;

impl TagHandler for ParameterHandler {
    fn handle(&mut self, _tag: &Handle, _printer: &mut StructuredPrinter) {}
    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
}
