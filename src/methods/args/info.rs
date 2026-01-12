use super::utils::{ensure_dir, resolve_alias};
use std::path::PathBuf;

use crate::models::Config;

pub struct InfoArgs {
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

    Ok(InfoArgs { path: tmpl_path })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn args_parse_info_returns_error_if_folder_does_not_exist() {
        let res = get_info_args(&["cpp".into(), "mario".into()], &Config::default());
        assert!(res.is_err());
    }
}
