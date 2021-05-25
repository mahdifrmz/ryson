use ryson::{Jerr, Json};
use str_macro::str;

#[test]
fn accepts_null(){
    let text = String::from("null");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Null);
}

#[test]
fn accepts_true(){
    let text = String::from("true");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Bool(true));
}

#[test]
fn accepts_false(){
    let text = String::from("false");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Bool(false));
}

#[test]
fn ignores_beginning_and_ending_spaces_and_new_lines(){
    let text = String::from(" \n  true\n  ");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Bool(true));
}

#[test]
fn throws_error_on_unknown_keyword(){
    let text = String::from("True");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(text));
}

#[test]
fn throws_unexpected_end_on_empty_string(){
    let text = String::from("");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn accepts_integers(){
    let text = String::from("1024");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Number(str!("1024")));
}

#[test]
fn error_on_non_digits_after_digits(){
    let text = String::from("4534h");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!("4534h")));
}

#[test]
fn error_on_non_zero_starting_with_zero(){
    let text = String::from("0916");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!("0916")));
}

#[test]
fn accepts_rationals(){
    let text = String::from("16.824");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Number(str!("16.824")));
}

#[test]
fn error_on_ending_dot(){
    let text = String::from("1624.");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!("1624.")));
}


#[test]
fn error_on_beginning_dot(){
    let text = String::from(".234567");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!(".234567")));
}

#[test]
fn error_on_multiple_dots(){
    let text = String::from("23.456.7");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!("23.456.7")));
}

#[test]
fn accepts_strings(){
    let text = String::from("\"hello world\"");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::String(str!("hello world")));
}

#[test]
fn unexpected_end_of_string(){
    let text = String::from("\"hello world");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn error_on_text_after_ending_quote(){
    let text = String::from("\"hello \nworld");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn escapes_back_slash_quote(){
    let text = String::from("\"a quote is a \\\" sign\"");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::String(str!("a quote is a \" sign")));
}

#[test]
fn escapes_double_back_slash(){
    let text = String::from("\"a backslash is a \\\\ sign\"");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::String(str!("a backslash is a \\ sign")));
}

#[test]
fn escapes_criagereturn_tab_newline_formfeed_backspace(){
    let text = String::from("\"escaped:\\n\\thello\\b\\ftext file\\r\"");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::String(str!("escaped:\n\thello\x08\x0Ctext file\r")));
}

#[test]
fn escapes_unicode(){
    let text = String::from("\"this is theta : \\u03F4\"");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::String(str!("this is theta : ϴ")));
}

#[test]
fn error_on_invalid_unicode(){
    let text = String::from("\"this is invalid : \\u93G4\"");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidUnicodeSequence(str!("93G4")));
}

#[test]
fn error_on_unknown_escape(){
    let text = String::from("\"I don't know \\a\"");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::UnknownEscape('a'));
}