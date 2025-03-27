use html2md::{Handle, NodeData};
use lazy_static::lazy_static;
use regex::Regex;

pub fn get_tag_name(tag: &Handle) -> Option<String> {
    match tag.data {
        NodeData::Element { ref name, .. } => Some(name.local.to_string()),
        _ => None,
    }
}

lazy_static! {
    static ref EXCESSIVE_WHITESPACE_PATTERN: Regex = Regex::new("\\s{2,}").unwrap();   // for HTML on-the-fly cleanup
}

pub fn get_text_content(tag: &Handle) -> String {
    let content = match tag.data {
        NodeData::Text { ref contents } => {
            let text = contents.borrow();
            if !(text.trim().is_empty()) {
                let minified_text = EXCESSIVE_WHITESPACE_PATTERN.replace_all(&text, " ");
                let minified_text = minified_text.trim_matches(|ch: char| ch == '\n' || ch == '\r');
                Some(minified_text.to_string())
            } else {
                None
            }
        }
        _ => None,
    };
    let children = tag.children.borrow();
    content
        .into_iter()
        .chain(children.iter().map(get_text_content))
        .collect()
}
