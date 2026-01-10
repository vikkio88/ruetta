use std::path::PathBuf;

use crate::{
    file::{exists, is_file, read_file, write_file},
    templates::ejs::parse_ejs,
};

const FRONT_MATTER_DELIM: &str = "---";

pub const INDEX_FILE: &str = "index.ruetta";
pub const RUETTA_EXT: &str = "ruetta";

pub struct WriteParams<'a> {
    pub name: &'a str,
    pub target_folder: &'a str,
    pub is_force: bool,
}

pub struct Template {
    pub additional_files: Vec<RuettaFile>,
    pub index: RuettaFile,
    pub path: PathBuf,
}

impl Template {
    pub fn load_from_folder(path: &PathBuf) -> Result<Template, String> {
        let main_path = path.clone();
        let path = if !is_file(path) {
            path.join(INDEX_FILE)
        } else {
            path.to_path_buf()
        };

        let content = read_file(&path)?;
        let (index, other_files) = RuettaIndex::from(&content)?;

        let mut additional_files = Vec::new();

        for p in &other_files {
            let file_path = main_path.join(format!("{}.{}", p, RUETTA_EXT));
            let content = read_file(&file_path)?;
            additional_files.push(RuettaFile::from(&content)?);
        }

        Ok(Template {
            index,
            path: main_path,
            additional_files,
        })
    }

    pub fn description(&self) -> Option<&String> {
        match &self.index.description {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn files(&self) -> Vec<&RuettaFile> {
        std::iter::once(&self.index)
            .chain(self.additional_files.iter())
            .collect()
    }

    // TODO: pass other variables as json? just to make sure we could pass vars
    // TODO: if has append param maybe just add if file exists "inject in hygen"
    pub fn write(&self, params: WriteParams) -> Result<String, String> {
        let mut written_paths = Vec::new();
        for file in self.files() {
            let to = file
                .to(params.name, params.target_folder)
                .map_err(|e| format!("Error whilst trying to get target name: {}", e))?;

            let body = file.body(params.name).map_err(|err| {
                format!(
                    "Cannot render body of the template: {}\nerror: {}",
                    self.path.display(),
                    err
                )
            })?;

            let target_path = &PathBuf::from(&to);
            // TODO: add check whether we are 'inject'
            if exists(target_path) && !params.is_force {
                return Err(format!(
                    "File '{}' exists already, if you want to overwrite use '--force' param to force.",
                    target_path.display()
                ));
            }

            write_file(&PathBuf::from(&to), &body).map_err(|err| {
                format!("Error whilst writing the file '{}'\n\terror: {}", to, err)
            })?;

            written_paths.push(to);
        }

        Ok(format!(
            "Files successfully created at:\n\t{}",
            written_paths.join("\n\t")
        ))
    }
}

pub struct RuettaIndex();

impl RuettaIndex {
    pub fn from(input: &str) -> Result<(RuettaFile, Vec<String>), String> {
        let (frontmatter, body) = ruetta_parts(input)?;
        let parsed = parse_frontmatter(frontmatter)?;
        let to = parsed
            .to
            .ok_or_else(|| "Missing 'to', file target.".to_string())?;
        let additional_files = parsed.files.unwrap_or_default();
        Ok((
            RuettaFile::new(to, parsed.description, body),
            additional_files,
        ))
    }
}

pub struct RuettaFile {
    to: String,
    pub description: Option<String>,
    body: String,
}

impl RuettaFile {
    pub fn from(input: &str) -> Result<RuettaFile, String> {
        let (frontmatter, body) = ruetta_parts(input)?;
        let parsed = parse_frontmatter(frontmatter)?;
        let to = parsed
            .to
            .ok_or_else(|| "Missing 'to', file target.".to_string())?;
        Ok(RuettaFile::new(to, parsed.description, body))
    }

    pub fn new(to: String, description: Option<String>, body: String) -> RuettaFile {
        RuettaFile {
            to,
            description,
            body,
        }
    }

    pub fn to(&self, name: &str, folder: &str) -> Result<String, String> {
        let values = serde_json::json!({"name": lowercase_first(name), "Name": capitalize_first(name), "folder": folder});
        parse_ejs(self.to.clone(), values)
    }

    pub fn body(&self, name: &str) -> Result<String, String> {
        let values =
            serde_json::json!({"name": lowercase_first(name), "Name": capitalize_first(name)});
        // TODO: this to extend the values and add additional vars
        // let mut values
        // let v = serde_json::json!({"yo": 1});
        // if let (Some(values_map), Some(v_map)) = (values.as_object_mut(), v.as_object()) {
        //     values_map.extend(v_map.clone());
        // }

        parse_ejs(self.body.clone(), values)
    }
}

fn ruetta_parts(input: &str) -> Result<(&str, String), String> {
    let mut parts = input.splitn(3, FRONT_MATTER_DELIM);
    parts.next();
    let frontmatter = parts.next().ok_or("Missing frontmatter")?;
    let body = parts.next().unwrap_or("").trim_start().to_string();

    Ok((frontmatter, body))
}

#[derive(serde::Deserialize)]
struct FrontMatterParsed {
    to: Option<String>,
    description: Option<String>,
    files: Option<Vec<String>>,
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
}
