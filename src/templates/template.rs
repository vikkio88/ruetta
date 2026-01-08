use serde_yml::Value;
use std::path::PathBuf;

use crate::{
    file::{is_file, read_file},
    templates::ejs::parse_ejs,
};

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

        parts.next();

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
    //TODO: this should be from folder, as there could be multiple files
    pub fn from_file(path: &PathBuf) -> Result<Template, String> {
        let path = if !is_file(path) {
            path.join(INDEX_FILE)
        } else {
            path.to_path_buf()
        };

        let content = read_file(&path)?;

        Template::from(&content)
    }

    pub fn to(&self, name: &str, folder: &str) -> Result<String, String> {
        let values = serde_json::json!({"name": lowercase_first(name), "Name": capitalize_first(name), "folder": folder});
        parse_ejs(self.to.clone(), values)
    }

    pub fn body(&self, name: &str) -> Result<String, String> {
        let values =
            serde_json::json!({"name": lowercase_first(name), "Name": capitalize_first(name)});
        parse_ejs(self.body.clone(), values)
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn lowercase_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn get_string(map: &serde_yml::Mapping, key: &str) -> Option<String> {
    map.get(&Value::String(key.into()))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
}

#[cfg(test)]
mod test {
    use super::*;
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

    #[test]
    fn to_computing_ejs() {
        let tpl = Template::from(
            "---
to: <%- folder %>/<%= name %>.cpp
description: |
    some description
    more stuff
---
body",
        )
        .unwrap();

        let to = tpl.to("name", "./some/folder").unwrap();
        assert_eq!(to, "./some/folder/name.cpp");
    }

    #[test]
    fn to_computing_ejs_capitalised() {
        let tpl = Template::from(
            "---
to: <%- folder %>/<%= Name %>.cpp
description: |
    some description
    more stuff
---
body",
        )
        .unwrap();

        let to = tpl.to("name", "./some/folder").unwrap();
        assert_eq!(to, "./some/folder/Name.cpp");
    }
}
