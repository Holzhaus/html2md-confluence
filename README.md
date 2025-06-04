# html2md-confluence [![License][license_badge]][license] [![Build Status][build_badge]][build] [![pre-commit.ci status][pre-commit_badge]][pre-commit]

**html2md-confluence** is a library that converts the [Confluence XHTML Storage Format][confluence_storage_format] into Markdown.
It is based on the [html2md][html2md].

## Usage

Use can use it like this:

```rust
use html2md_confluence::{ConfluencePageId, ConfluenceServer, JiraServer, ParseOptions, parse_confluence};
use std::str::FromStr;

pub fn convert(source: &str) -> String {
    let options = ParseOptions::default()
        .with_confluence_server(
            ConfluenceServer::from_str("https://example.com/confluence").unwrap()
        )
        .with_default_page_id(ConfluencePageId::from(12345))
        .with_default_space_key("CONFL")
        .with_jira_server(
            "144880e9-a1111-333f-9412-ed999a9999fa".to_string(),
            JiraServer::from_str("http://jira.atlassian.com").unwrap()
        );
    parse_confluence(&buffer, &options)
}
```

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

[html2md]: https://crates.io/crates/html2md
[confluence_storage_format]: https://confluence.atlassian.com/doc/confluence-storage-format-790796544.html
[license]: https://github.com/Holzhaus/html2md-confluence/blob/main/COPYING
[license_badge]: https://img.shields.io/github/license/Holzhaus/html2md-confluence
[build]: https://github.com/Holzhaus/html2md-confluence/actions?query=branch%3Amain
[build_badge]: https://img.shields.io/github/actions/workflow/status/Holzhaus/html2md-confluence/build.yml?branch=main
[pre-commit]: https://results.pre-commit.ci/latest/github/Holzhaus/html2md-confluence/main
[pre-commit_badge]: https://results.pre-commit.ci/badge/github/Holzhaus/html2md-confluence/main.svg
