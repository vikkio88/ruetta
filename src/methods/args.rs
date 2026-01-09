use std::path::{Path, PathBuf};

use crate::{file::is_dir, models::Config};

pub struct MakeArgs {
    pub language: String,
    pub template: String,
    pub template_path: PathBuf,
    pub target_folder: String,
    pub name: String,
    pub is_dry_run: bool,
    pub is_force: bool,
}

fn resolve_alias(cfg: &Config, value: &str) -> String {
    cfg.aliases
        .get(value)
        .cloned()
        .unwrap_or_else(|| value.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn resolve_alias_works_with_multiple_keys() {
        let cfg = Config {
            folder: "some".into(),
            aliases: HashMap::from([
                ("c".into(), "component".into()),
                ("cmp".into(), "component".into()),
                ("comp".into(), "component".into()),
                ("sv".into(), "svelte".into()),
                ("cls".into(), "class".into()),
            ]),
        };

        assert_eq!("component", resolve_alias(&cfg, "component"));
        assert_eq!("component", resolve_alias(&cfg, "c"));
        assert_eq!("component", resolve_alias(&cfg, "comp"));
        assert_eq!("component", resolve_alias(&cfg, "cmp"));
        assert_eq!("svelte", resolve_alias(&cfg, "sv"));
        assert_eq!("svelte", resolve_alias(&cfg, "svelte"));
        assert_eq!("class", resolve_alias(&cfg, "cls"));
        assert_eq!("class", resolve_alias(&cfg, "class"));
    }
}

fn ensure_dir(path: &Path, err: String) -> Result<(), String> {
    if is_dir(&path.to_path_buf()) {
        Ok(())
    } else {
        Err(err)
    }
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
    for arg in rest {
        match arg.as_str() {
            "--dry-run" => is_dry_run = true,
            "--force" => is_force = true,
            _ => {}
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
        language,
        template,
        template_path: tmpl_path,
        target_folder: target_folder.clone(),
        name: name.clone(),
        is_dry_run,
        is_force,
    })
}

pub struct InfoArgs {
    language: String,
    template: String,
    pub path: PathBuf,
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

    Ok(InfoArgs {
        language,
        template,
        path: tmpl_path,
    })
}

#[test]
fn args_parse_info_returns_error_if_folder_does_not_exist() {
    let res = get_info_args(&["cpp".into(), "mario".into()], &Config::default());
    assert!(res.is_err());
}
