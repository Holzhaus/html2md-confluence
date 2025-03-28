mod expand;

use crate::util::get_tag_name;
use html2md::{Handle, StructuredPrinter, TagHandler, common::get_tag_attr};

#[derive(Default)]
pub struct StructuredMacroHandler {
    macro_specific_handler: Option<Box<dyn TagHandler>>,
}

impl TagHandler for StructuredMacroHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        debug_assert_eq!(get_tag_name(tag).unwrap(), "ac:structured-macro");

        self.macro_specific_handler = match get_tag_attr(tag, "ac:name").as_deref() {
            Some("expand") => Some(Box::new(expand::ExpandMacroHandler::new())),
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
