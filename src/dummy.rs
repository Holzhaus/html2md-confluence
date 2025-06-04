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
