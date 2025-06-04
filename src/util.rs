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

use html2md::{Handle, NodeData};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ConfluencePageId(usize);

impl From<usize> for ConfluencePageId {
    fn from(page_id: usize) -> Self {
        Self(page_id)
    }
}

impl fmt::Display for ConfluencePageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ConfluencePageId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>().map(Self::from)
    }
}

#[derive(Debug, Clone)]
pub struct ConfluenceServer {
    base_url: String,
}

impl ConfluenceServer {
    pub fn attachment_url<S: AsRef<str>>(&self, page_id: &ConfluencePageId, filename: S) -> String {
        format!(
            "{base_url}/download/attachments/{page_id}/{filename}",
            base_url = self.base_url,
            filename = urlencoding::encode(filename.as_ref()),
        )
    }

    pub fn user_url_with_name<S: AsRef<str>>(&self, username: S) -> String {
        format!(
            "{base_url}/users/viewuserprofile.action?username={username}",
            base_url = self.base_url,
            username = urlencoding::encode(username.as_ref()),
        )
    }

    pub fn user_url_with_key<S: AsRef<str>>(&self, userkey: S) -> String {
        format!(
            "{base_url}/users/viewuserprofile.action?userkey={userkey}",
            base_url = self.base_url,
            userkey = urlencoding::encode(userkey.as_ref()),
        )
    }

    pub fn page_url_with_space_and_title<S: AsRef<str>, T: AsRef<str>>(
        &self,
        space: S,
        page_title: T,
    ) -> String {
        format!(
            "{base_url}/display/{space}/{page_title}",
            base_url = self.base_url,
            space = urlencoding::encode(space.as_ref()),
            page_title = urlencoding::encode(page_title.as_ref()),
        )
    }
}

impl FromStr for ConfluenceServer {
    type Err = ();

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            base_url: s.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct JiraServer {
    base_url: String,
}

impl JiraServer {
    pub fn issue_url<S: AsRef<str>>(&self, key: S) -> String {
        format!("{}/browse/{}", self.base_url, key.as_ref())
    }

    pub fn jql_url<S: AsRef<str>>(&self, jql: S) -> String {
        let jql_encoded = urlencoding::encode(jql.as_ref());
        format!("{}/issues/?jql={jql_encoded}", self.base_url)
    }
}

impl FromStr for JiraServer {
    type Err = ();

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            base_url: s.to_string(),
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct JiraServerMap(HashMap<String, JiraServer>);

impl JiraServerMap {
    pub fn insert(&mut self, server_id: String, server: JiraServer) -> Option<JiraServer> {
        self.0.insert(server_id, server)
    }

    pub fn by_id<S: AsRef<str>>(&self, server_id: S) -> Option<&JiraServer> {
        self.0.get(server_id.as_ref())
    }
}

impl From<&[(&str, &str)]> for JiraServerMap {
    fn from(servers: &[(&str, &str)]) -> Self {
        Self(
            servers
                .iter()
                .map(|(server_id, base_url)| {
                    (
                        server_id.to_string(),
                        JiraServer::from_str(base_url).unwrap(),
                    )
                })
                .collect(),
        )
    }
}

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

#[cfg(test)]
#[macro_export]
macro_rules! markdown_assert_eq {
    ($html:expr, $markdown:expr) => {
        use $crate::{ParseOptions, parse_confluence};

        let options = ParseOptions::default();
        let md = parse_confluence($html, &options);
        assert_eq!(md, $markdown);
    };
}
