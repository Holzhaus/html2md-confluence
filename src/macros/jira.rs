use crate::util::{get_tag_name, get_text_content};
use html2md::{Handle, StructuredPrinter, TagHandler, common::get_tag_attr};
use std::collections::HashMap;
use std::str::FromStr;

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
    use super::*;
    use html2md::{TagHandlerFactory, parse_html_custom};

    struct Factory(JiraServerMap);
    impl TagHandlerFactory for Factory {
        fn instantiate(&self) -> Box<dyn TagHandler> {
            Box::new(JiraMacroHandler::with_servers(self.0.clone()))
        }
    }

    fn get_handlers() -> HashMap<String, Box<(dyn TagHandlerFactory + 'static)>> {
        let mut handlers: HashMap<String, Box<(dyn TagHandlerFactory + 'static)>> = HashMap::new();
        let servers = JiraServerMap::from(
            [(
                "144880e9-a1111-333f-9412-ed999a9999fa",
                "http://jira.atlassian.com",
            )]
            .as_slice(),
        );
        handlers.insert(
            String::from("ac:structured-macro"),
            Box::new(Factory(servers)),
        );
        handlers
    }

    #[test]
    fn test_jql_query() {
        let handlers = get_handlers();
        let md = parse_html_custom(
            r#"
  <ac:structured-macro ac:name="jira">
  <ac:parameter ac:name="columns">key,summary,type,created,assignee,status</ac:parameter>
  <ac:parameter ac:name="server">Atlassian JIRA</ac:parameter>
  <ac:parameter ac:name="serverId">144880e9-a1111-333f-9412-ed999a9999fa</ac:parameter>
    <ac:parameter ac:name="jqlQuery">project = CONF AND component = documentation AND resolution = unresolved</ac:parameter>
  </ac:structured-macro>"#,
            &handlers,
        );
        assert_eq!(
            md,
            "[``project = CONF AND component = documentation AND resolution = unresolved``](http://jira.atlassian.com/issues/?jql=project%20%3D%20CONF%20AND%20component%20%3D%20documentation%20AND%20resolution%20%3D%20unresolved)"
        )
    }

    #[test]
    fn test_issue_key() {
        let handlers = get_handlers();
        let md = parse_html_custom(
            r#"
  <ac:structured-macro ac:name="jira">
  <ac:parameter ac:name="columns">key,summary,type,created,assignee,status</ac:parameter>
  <ac:parameter ac:name="server">Atlassian JIRA</ac:parameter>
  <ac:parameter ac:name="serverId">144880e9-a1111-333f-9412-ed999a9999fa</ac:parameter>
  <ac:parameter ac:name="key">CONF-1234</ac:parameter>
  </ac:structured-macro>"#,
            &handlers,
        );
        assert_eq!(
            md,
            "[CONF-1234](http://jira.atlassian.com/browse/CONF-1234)"
        )
    }
}
