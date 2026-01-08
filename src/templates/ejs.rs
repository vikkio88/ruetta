use dojang::Dojang;

//TODO: maybe write my own ejs parser as I only need bits of things

pub fn parse_ejs(raw: String, values: serde_json::Value) -> Result<String, String> {
    let mut parser = Dojang::new();
    _ = parser.add("tmp".into(), raw);
    parser.render("tmp", values)
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ejs_parser_parses_correctly() {
        let res = parse_ejs(
            "some <%= stuff %> and things".into(),
            serde_json::json!({"stuff": "stuff"}),
        )
        .unwrap();

        assert_eq!(res, "some stuff and things");
    }

    #[test]
    #[ignore = "deciding whether to build my own ejs parser"]
    fn ejs_parser_parses_fails_if_not_correct_values() {
        let res = parse_ejs(
            "some <%= stuff %> and things".into(),
            serde_json::json!({"a": "stuff"}),
        )
        .unwrap();

        assert_eq!(res, "some stuff and things");
    }

    #[test]
    fn ejs_parser_does_not_escape_folder_values() {
        let res = parse_ejs(
            "some folder <%- folder %>".into(),
            serde_json::json!({"folder": "./ciao/blap/blip"}),
        )
        .unwrap();

        assert_eq!(res, "some folder ./ciao/blap/blip");
    }
}
