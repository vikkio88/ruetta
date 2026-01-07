use std::path::PathBuf;

use crate::file::{is_file, read_file};

const KEY_TO: &str = "to";
const KEY_DESCRIPTION: &str = "description";
const FRONT_MATTER_DELIM: &str = "---";

pub const INDEX_FILE: &str = "index.ruetta";

pub struct Template {
    to: String,
    pub description: Option<String>,
    body: String,
}

impl Template {
    pub fn from(input: &str) -> Result<Template, String> {
        let mut parts = input.splitn(3, FRONT_MATTER_DELIM);

        parts.next(); // before first ---

        let frontmatter = parts.next().ok_or("Missing frontmatter")?;

        let body = parts.next().unwrap_or("").trim_start().to_string();

        let mut to: Option<String> = None;
        let mut description: Option<String> = None;

        for line in frontmatter.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (key, value) = line.split_once(':').ok_or("Invalid frontmatter line")?;

            let value = value.trim().to_string();

            match key.trim() {
                KEY_TO => to = Some(value),
                KEY_DESCRIPTION => description = Some(value),
                _ => {}
            }
        }

        let to = to.ok_or_else(|| "Missing 'to', file target.".to_string())?;

        Ok(Template {
            to,
            description,
            body,
        })
    }

    pub fn from_file(path: &PathBuf) -> Result<Template, String> {
        let path = if !is_file(path) {
            path.join(INDEX_FILE)
        } else {
            path.to_path_buf()
        };

        let content = read_file(&path)?;

        Template::from(&content)
    }
}

#[test]
fn build_template_from_string() {
    let tpl = Template::from(
        "---
to: ciao.cpp
description: some description
---
body",
    )
    .unwrap();

    assert_eq!(tpl.to, "ciao.cpp");
    assert_eq!(tpl.description.as_deref(), Some("some description"));
    assert_eq!(tpl.body, "body");
}
