use std::{collections::HashMap, path::PathBuf};

use crate::{
    file::{exists, is_file, read_file, write_file},
    templates::files::{RuettaFile, RuettaIndex},
};

pub const INDEX_FILE: &str = "index.ruetta";
pub const RUETTA_EXT: &str = "ruetta";

pub struct WriteParams<'a> {
    pub name: &'a str,
    pub target_folder: &'a str,
    pub is_force: bool,
    pub vars: Option<HashMap<String, String>>,
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
    // TODO: move this method to file itself
    pub fn write(&self, params: WriteParams) -> Result<String, String> {
        let mut written_paths = Vec::new();
        for file in self.files() {
            let to = file
                .to(params.name, params.target_folder)
                .map_err(|e| format!("Error whilst trying to get target name: {}", e))?;

            let target_path = &PathBuf::from(&to);
            let is_appending = file.append.is_some_and(|x| x) || file.append_after.is_some();

            if exists(target_path) && !params.is_force && !is_appending {
                return Err(format!(
                    "File '{}' exists already, if you want to overwrite use '--force' param to force.",
                    target_path.display()
                ));
            }

            let mut body = file.body(params.name, &params.vars).map_err(|err| {
                format!(
                    "Cannot render body of the template: {}\nerror: {}",
                    self.path.display(),
                    err
                )
            })?;

            if exists(target_path) && is_appending {
                let existing_body = read_file(target_path)?;
                if let Some(pattern) = file.append_after.as_deref() {
                    body = if let Some((before, after)) = existing_body.split_once(pattern) {
                        format!("{}{}{}", before, body, after)
                    } else {
                        format!("{}{}", existing_body, body)
                    };
                } else if file.append.is_some_and(|is_appending| is_appending) {
                    body = format!("{}{}", existing_body, body);
                }
            }

            write_file(&PathBuf::from(&to), &body).map_err(|err| {
                format!("Error whilst writing the file '{}'\n\terror: {}", to, err)
            })?;

            written_paths.push(format!(
                "{} ({})",
                to,
                if is_appending { "updated" } else { "created" }
            ));
        }

        Ok(format!(
            "Files successfully generated:\n\t{}",
            written_paths.join("\n\t")
        ))
    }
}
