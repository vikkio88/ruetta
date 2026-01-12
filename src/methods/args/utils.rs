use std::{collections::HashMap, path::Path};

use crate::{file::is_dir, models::Config};

pub fn resolve_alias(cfg: &Config, value: &str) -> String {
    cfg.aliases
        .get(value)
        .cloned()
        .unwrap_or_else(|| value.to_string())
}

pub fn parse_vars(input: &str) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();

    if input.is_empty() {
        return Ok(map);
    }

    for pair in input.split(',') {
        let (key, value) = pair
            .split_once(':')
            .ok_or_else(|| format!("Invalid var '{}', expected key:value", pair))?;

        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}

pub fn ensure_dir(path: &Path, err: String) -> Result<(), String> {
    if is_dir(&path.to_path_buf()) {
        Ok(())
    } else {
        Err(err)
    }
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

    #[test]
    fn parse_vars_working_correctly() {
        let vars = parse_vars("some:things,are:not,great:1").unwrap();
        assert_eq!(
            vars,
            HashMap::from([
                ("some".to_string(), "things".to_string()),
                ("are".to_string(), "not".to_string()),
                ("great".to_string(), "1".to_string()),
            ])
        );
    }
}
