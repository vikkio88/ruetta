use super::utils::resolve_alias;
use std::path::PathBuf;

use crate::models::Config;

pub struct CreateArgs {
    pub language: String,
    pub template: String,
    pub path: PathBuf,
}

pub fn get_create_args(args: &[String], cfg: &Config) -> Result<CreateArgs, String> {
    let [lang, tmpl] = args else {
        return Err("Missing argument(s):
Example usage:
    ruetta create <language> <template>
e.g. ruetta create svelte component"
            .to_string());
    };

    //TODO: add name optional

    let language = resolve_alias(cfg, lang);
    let base = PathBuf::from(&cfg.folder).join(&language);

    let template = resolve_alias(cfg, tmpl);
    let tmpl_path = base.join(&template);

    // TODO: maybe ensure_dir here but reverse?

    Ok(CreateArgs {
        language,
        template,
        path: tmpl_path,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn args_parse_info_returns_error_if_folder_does_not_exist() {
        let res = get_create_args(&["cpp".into(), "mario".into()], &Config::default());
        assert!(res.is_err());
    }
}
