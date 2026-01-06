use std::path::{Path, PathBuf};

use crate::{file::is_dir, models::Config};

pub struct MakeArgs {
    pub language: String,
    pub template: String,
    pub target_folder: String,
    pub name: String,
}

fn resolve_alias(cfg: &Config, value: &str) -> String {
    cfg.aliases
        .get(value)
        .cloned()
        .unwrap_or_else(|| value.to_string())
}

fn ensure_dir(path: &Path, err: String) -> Result<(), String> {
    if is_dir(&path.to_path_buf()) {
        Ok(())
    } else {
        Err(err)
    }
}
pub fn get_make_args(args: &[String], cfg: &Config) -> Result<MakeArgs, String> {
    let [lang, tmpl, name, target_folder] = args else {
        return Err("Missing argument(s):
Example usage:
    ruetta make <language> <template> <name> <target_folder>
e.g. ruetta make svelte component Counter src/lib"
            .to_string());
    };

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
        language,
        template,
        target_folder: target_folder.clone(),
        name: name.clone(),
    })
}

pub struct InfoArgs {
    pub language: String,
    pub template: String,
}

pub fn get_info_args(args: &[String], cfg: &Config) -> Result<InfoArgs, String> {
    let [lang, tmpl] = args else {
        return Err("Missing argument(s):
Example usage:
    ruetta info <language> <template>
e.g. ruetta info svelte component"
            .to_string());
    };

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

    Ok(InfoArgs { language, template })
}
