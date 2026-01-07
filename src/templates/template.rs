use serde_yml::Value;
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

        if let Ok(Value::Mapping(map)) = serde_yml::from_str::<Value>(frontmatter) {
            to = get_string(&map, KEY_TO);
            description = get_string(&map, KEY_DESCRIPTION);
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

fn get_string(map: &serde_yml::Mapping, key: &str) -> Option<String> {
    map.get(&Value::String(key.into()))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
}

#[test]
fn build_template_from_string() {
    let tpl = Template::from(
        "---
to: ciao.cpp
description: |
    some description
    more stuff
---
body",
    )
    .unwrap();

    assert_eq!(tpl.to, "ciao.cpp");
    assert_eq!(
        tpl.description.as_deref(),
        Some("some description\nmore stuff")
    );
    assert_eq!(tpl.body, "body");
}
