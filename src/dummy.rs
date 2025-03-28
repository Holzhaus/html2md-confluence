use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory};

pub struct RecursiveDummyHandler;
impl TagHandler for RecursiveDummyHandler {
    fn handle(&mut self, _tag: &Handle, _printer: &mut StructuredPrinter) {}
    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
    fn skip_descendants(&self) -> bool {
        true
    }
}

pub struct RecursiveDummyHandlerFactory;
impl TagHandlerFactory for RecursiveDummyHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::new(RecursiveDummyHandler {})
    }
}
