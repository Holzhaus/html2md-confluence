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

use crate::util::{JiraServerMap, get_tag_name, get_text_content};
use html2md::{Handle, StructuredPrinter, TagHandler, common::get_tag_attr};

#[derive(Debug, Clone)]
pub struct JiraMacroHandler {
    servers: JiraServerMap,
}

impl JiraMacroHandler {
    pub fn with_servers(servers: JiraServerMap) -> Self {
        Self { servers }
    }
}

impl TagHandler for JiraMacroHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let mut key = None;
        let mut jql = None;
        let mut server = None;

        let children = tag.children.borrow();
        let params = children.iter().filter_map(|child| {
            if get_tag_name(child).is_some_and(|name| name == "ac:parameter") {
                get_tag_attr(child, "ac:name").map(|value| (value, child))
            } else {
                None
            }
        });

        for (param_name, param) in params {
            match param_name.as_str() {
                "key" => key = Some(get_text_content(param)),
                "jqlQuery" => jql = Some(get_text_content(param)),
                "serverId" => server = self.servers.by_id(get_text_content(param)),
                _ => (),
            }
        }

        let Some(server) = server else {
            return;
        };

        let link = jql
            .map(|query| {
                let url = server.jql_url(&query);
                format!("[``{query}``]({url})")
            })
            .or_else(|| {
                key.map(|k| {
                    let url = server.issue_url(&k);
                    format!("[{k}]({url})")
                })
            });
        if let Some(link) = link {
            printer.append_str(&link);
        }
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}

    fn skip_descendants(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::{JiraServer, ParseOptions, parse_confluence};
    use std::str::FromStr;

    macro_rules! markdown_assert_eq {
        ($html:expr, $markdown:expr) => {
            let options = ParseOptions::default().with_jira_server(
                "144880e9-a1111-333f-9412-ed999a9999fa".to_string(),
                JiraServer::from_str("http://jira.atlassian.com").unwrap(),
            );
            let md = parse_confluence($html, &options);
            assert_eq!(md, $markdown);
        };
    }

    #[test]
    fn test_jql_query() {
        markdown_assert_eq!(
            r#"
  <ac:structured-macro ac:name="jira">
  <ac:parameter ac:name="columns">key,summary,type,created,assignee,status</ac:parameter>
  <ac:parameter ac:name="server">Atlassian JIRA</ac:parameter>
  <ac:parameter ac:name="serverId">144880e9-a1111-333f-9412-ed999a9999fa</ac:parameter>
    <ac:parameter ac:name="jqlQuery">project = CONF AND component = documentation AND resolution = unresolved</ac:parameter>
  </ac:structured-macro>"#,
            "[``project = CONF AND component = documentation AND resolution = unresolved``](http://jira.atlassian.com/issues/?jql=project%20%3D%20CONF%20AND%20component%20%3D%20documentation%20AND%20resolution%20%3D%20unresolved)"
        );
    }

    #[test]
    fn test_issue_key() {
        markdown_assert_eq!(
            r#"
  <ac:structured-macro ac:name="jira">
  <ac:parameter ac:name="columns">key,summary,type,created,assignee,status</ac:parameter>
  <ac:parameter ac:name="server">Atlassian JIRA</ac:parameter>
  <ac:parameter ac:name="serverId">144880e9-a1111-333f-9412-ed999a9999fa</ac:parameter>
  <ac:parameter ac:name="key">CONF-1234</ac:parameter>
  </ac:structured-macro>"#,
            "[CONF-1234](http://jira.atlassian.com/browse/CONF-1234)"
        );
    }
}
