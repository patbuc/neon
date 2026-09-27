use crate::compiler::tokens_to_json;

#[test]
fn renders_token_stream_as_json() {
    let source = "var s = \"a\\n${1}\";";

    let json = tokens_to_json(source);

    assert_eq!(
        json,
        r#"[{"kind":"Var","line":1,"column":1,"lexeme":"var"},{"kind":"Identifier","line":1,"column":5,"lexeme":"s"},{"kind":"Equal","line":1,"column":7,"lexeme":"="},{"kind":"StringStart","line":1,"column":9,"lexeme":"\"a\\n${"},{"kind":"Number","line":1,"column":15,"lexeme":"1"},{"kind":"StringEnd","line":1,"column":16,"lexeme":"}\""},{"kind":"Semicolon","line":1,"column":18,"lexeme":";"}]"#
    );
}

#[test]
fn escapes_control_characters_and_passes_through_non_ascii() {
    let source = "\"\u{1}é\"";

    let json = tokens_to_json(source);

    assert_eq!(
        json,
        r#"[{"kind":"String","line":1,"column":1,"lexeme":"\"\u0001é\""}]"#
    );
}
