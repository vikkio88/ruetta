use super::utils::{ensure_dir, parse_vars, resolve_alias};
use std::{collections::HashMap, path::PathBuf};

use crate::models::Config;

pub struct MakeArgs {
    pub template_path: PathBuf,
    pub target_folder: String,
    pub name: String,
    pub is_dry_run: bool,
    pub is_force: bool,
    pub vars: Option<HashMap<String, String>>,
}

pub fn get_make_args(args: &[String], cfg: &Config) -> Result<MakeArgs, String> {
    let [lang, tmpl, name, target_folder, rest @ ..] = args else {
        return Err("Missing argument(s):
Example usage:
    ruetta make <language> <template> <name> <target_folder>
e.g. ruetta make svelte component Counter src/lib"
            .to_string());
    };
    let mut is_dry_run = false;
    let mut is_force = false;
    let mut vars: Option<HashMap<String, String>> = None;
    for arg in rest {
        match arg.as_str() {
            "--dry-run" => is_dry_run = true,
            "--force" => is_force = true,
            _ if arg.starts_with("--vars=") => {
                let raw = arg.strip_prefix("--vars=").unwrap().trim_matches('"');
                vars = Some(parse_vars(raw)?);
            }
            unknown => {
                return Err(format!("Unknown option '{}'", unknown));
            }
        }
    }

    let language = resolve_alias(cfg, lang);
    let base = PathBuf::from(&cfg.folder).join(&language);

    ensure_dir(
        &base,
        format!(
            "language '{}' does not exist in template folder '{}'",
            language, cfg.folder
        ),
    )?;

    let template = resolve_alias(cfg, tmpl);
    let tmpl_path = base.join(&template);

    ensure_dir(
        &tmpl_path,
        format!(
            "template '{}' does not exist in template folder '{}' for language '{}'",
            template, cfg.folder, language
        ),
    )?;

    Ok(MakeArgs {
        template_path: tmpl_path,
        target_folder: target_folder.clone(),
        name: name.clone(),
        is_dry_run,
        is_force,
        vars,
    })
}
