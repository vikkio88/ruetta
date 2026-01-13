use std::collections::HashMap;

use crate::templates::ejs::parse_ejs;
const FRONT_MATTER_DELIM: &str = "---";

fn ruetta_parts(input: &str) -> Result<(&str, String), String> {
    let mut parts = input.splitn(3, FRONT_MATTER_DELIM);
    parts.next();
    let frontmatter = parts.next().ok_or("Missing frontmatter")?;
    let body = parts.next().unwrap_or("").trim_start().to_string();

    Ok((frontmatter, body))
}

pub struct RuettaIndex();
impl RuettaIndex {
    pub fn from(input: &str) -> Result<(RuettaFile, Vec<String>), String> {
        let (frontmatter, body) = ruetta_parts(input)?;
        let mut parsed_fm = parse_frontmatter(frontmatter)?;

        let to = parsed_fm
            .to
            .take()
            .ok_or_else(|| "Missing 'to', file target.".to_string())?;

        let additional_files = parsed_fm.files.take().unwrap_or_default();

        let file = RuettaFile::new(to, body, parsed_fm);

        Ok((file, additional_files))
    }
}

pub struct RuettaFile {
    to: String,
    pub description: Option<String>,
    pub append_after: Option<String>,
    pub append: Option<bool>,
    pub exclude_if: Option<String>,
    body: String,
}

impl RuettaFile {
    pub fn from(input: &str) -> Result<RuettaFile, String> {
        let (frontmatter, body) = ruetta_parts(input)?;
        let mut parsed_fm = parse_frontmatter(frontmatter)?;

        let to = parsed_fm
            .to
            .take()
            .ok_or_else(|| "Missing 'to', file target.".to_string())?;

        Ok(RuettaFile::new(to, body, parsed_fm))
    }

    pub fn new(to: String, body: String, fm: FrontMatterParsed) -> RuettaFile {
        RuettaFile {
            to,
            description: fm.description,
            append_after: fm.append_after,
            append: fm.append,
            exclude_if: fm.exclude_if,
            body,
        }
    }

    pub fn should_exclude(&self, vars: Option<&HashMap<String, String>>) -> bool {
        self.exclude_if
            .as_deref()
            .and_then(|k| vars.map(|v| v.contains_key(k)))
            .unwrap_or(false)
    }

    pub fn to(&self, name: &str, folder: &str) -> Result<String, String> {
        let values = serde_json::json!({"name": lowercase_first(name), "Name": capitalize_first(name), "folder": folder});
        parse_ejs(self.to.clone(), values)
    }

    pub fn body(
        &self,
        name: &str,
        vars: &Option<HashMap<String, String>>,
    ) -> Result<String, String> {
        let mut values = serde_json::json!({
            "name": lowercase_first(name),
            "Name": capitalize_first(name),
        });

        if let Some(vars) = vars
            && let Some(values_map) = values.as_object_mut()
        {
            for (k, v) in vars {
                values_map.insert(k.clone(), serde_json::Value::String(v.clone()));
            }
        }

        parse_ejs(self.body.clone(), values)
    }
}

#[derive(serde::Deserialize)]
pub struct FrontMatterParsed {
    to: Option<String>,
    description: Option<String>,
    append_after: Option<String>,
    append: Option<bool>,
    files: Option<Vec<String>>,
    exclude_if: Option<String>,
}

fn parse_frontmatter(frontmatter: &str) -> Result<FrontMatterParsed, String> {
    serde_yml::from_str::<FrontMatterParsed>(frontmatter).map_err(|e| e.to_string())
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn build_template_from_string() {
        let tpl = RuettaFile::from(
            "---
to: ciao.cpp
description: |
    some description
    more stuff
files:
    - ciao
---
body",
        )
        .unwrap();

        assert_eq!(tpl.to, "ciao.cpp");
        assert_eq!(
            tpl.description.as_deref(),
            Some("some description\nmore stuff\n")
        );
        assert_eq!(tpl.body, "body");
    }

    #[test]
    fn to_computing_ejs() {
        let tpl = RuettaFile::from(
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
        let tpl = RuettaFile::from(
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

    #[test]
    fn parse_list_of_files() {
        let (fm, _) = ruetta_parts(
            "---
to: <%- folder %>/<%= Name %>.cpp
description: |
    some description
    more stuff
files:
    - stuff
    - things
---
body",
        )
        .unwrap();

        let parsed = parse_frontmatter(fm).unwrap();
        assert_eq!(parsed.files.unwrap(), ["stuff", "things"])
    }

    #[test]
    fn parse_append() {
        let (fm, _) = ruetta_parts(
            "---
to: <%- folder %>/<%= Name %>.cpp
append: true
---
body",
        )
        .unwrap();

        let parsed = parse_frontmatter(fm).unwrap();
        assert_eq!(parsed.append.unwrap(), true);
        assert!(parsed.append_after.is_none());
    }

    #[test]
    fn parse_append_after() {
        let (fm, _) = ruetta_parts(
            "---
to: <%- folder %>/<%= Name %>.cpp
append_after: something
---
body",
        )
        .unwrap();

        let parsed = parse_frontmatter(fm).unwrap();
        assert!(parsed.append.is_none());
        assert_eq!(parsed.append_after.unwrap(), "something".to_string());
    }

    #[test]
    fn parse_exclude_if() {
        let (fm, _) = ruetta_parts(
            "---
to: <%- folder %>/<%= Name %>.cpp
exclude_if: something
---
body",
        )
        .unwrap();

        let parsed = parse_frontmatter(fm).unwrap();
        assert_eq!(parsed.exclude_if.unwrap(), "something".to_string());
    }
}
