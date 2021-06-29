use std::collections::HashMap;

use ryson::{Json,Jerr};
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
fn throws_error_on_unknown_keyword(){
    let text = String::from("True");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedChar(0));
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
    assert_eq!(jerr,Jerr::ExpectedEnd(4));
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
    assert_eq!(jerr,Jerr::UnexpectedChar(0));
}

#[test]
fn error_on_multiple_dots(){
    let text = String::from("23.456.7");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedEnd(6));
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

#[test]
fn single_element_array(){
    let text = String::from("[false]");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Array(vec![Json::Bool(false)]));
}

#[test]
fn multi_element_array(){
    let text = String::from(
        "[true,1444,\"third element\"]"
    );
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Array(vec![
        Json::Bool(true),
        Json::Number(str!("1444")),
        Json::String(str!("third element"))
    ]));
}

#[test]
fn ignore_white_space_newline(){
    let text = String::from(
        "[true,  1444\n,  \"third element\"\n\n  ]"
    );
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Array(vec![
        Json::Bool(true),
        Json::Number(str!("1444")),
        Json::String(str!("third element"))
    ]));
}

#[test]
fn error_on_not_ended_array(){
    let text = String::from(
        "[true,  1444\n,  \"third element\"\n\n  "
    );
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn error_on_multiple_commas(){
    let text = String::from(
        "[true,  1444\n, , \"third element\"\n\n  "
    );
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedValue(15));
}

#[test]
fn error_on_multiple_value(){
    let text = String::from(
        "[true,  1444\n \"third element\"\n\n  "
    );
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedCommaOrEnd(14));
}

#[test]
fn accept_nested_arrays(){
    let text = String::from("[\n   [false]\n]");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Array(vec![Json::Array(vec![Json::Bool(false)])]));
}

#[test]
fn accepts_empty_array(){
    let text = String::from("[]");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Array(vec![]));
}

#[test]
fn accepts_single_field_objects(){
    let text = String::from("{\"port\":8080}");
    let json = Json::parse(&text).unwrap();
    let mut map = HashMap::new();
    map.insert(str!("port"), Json::Number(str!("8080")));
    assert_eq!(json,Json::Object(map));
}

#[test]
fn error_on_missing_colon(){
    let text = String::from("{\"port\",8080}");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedColon(7));
}

#[test]
fn error_on_invalid_property_identifier(){
    let text = String::from("{3,8080}");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedProperty(1));
}

#[test]
fn error_on_missing_property(){
    let text = String::from("{\"host\":}");
    let jerr = Json::parse(&text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedValue(8));
}

#[test]
fn accepts_multi_field_objects(){
    let text = String::from("{\"port\":80,\n\"host\":\"localhost\"}");
    let json = Json::parse(&text).unwrap();

    let mut map = HashMap::new();
    map.insert(str!("port"), Json::Number(str!("80")));
    map.insert(str!("host"), Json::String(str!("localhost")));

    assert_eq!(json,Json::Object(map));
}

#[test]
fn accepts_object_array_property(){
    let text = String::from("{\"port\":80,\n\"host\":[\"localhost\",true]}");
    let json = Json::parse(&text).unwrap();

    let mut map = HashMap::new();
    let arr = vec![Json::String(str!("localhost")),Json::Bool(true)];
    map.insert(str!("port"), Json::Number(str!("80")));
    map.insert(str!("host"), Json::Array(arr));

    assert_eq!(json,Json::Object(map));
}

#[test]
fn accepts_nested_objects(){
    let text = String::from("{\"port\":80,\n\"host\":{\"localhost\":true}}");
    let json = Json::parse(&text).unwrap();

    let mut map = HashMap::new();
    let mut inner_map = HashMap::new();
    inner_map.insert(str!("localhost"), Json::Bool(true));
    map.insert(str!("port"), Json::Number(str!("80")));
    map.insert(str!("host"), Json::Object(inner_map));

    assert_eq!(json,Json::Object(map));
}

#[test]
fn accepts_array_with_object_element(){
    let text = String::from("[{\"version\":\"1.10.3\"}]");
    let json = Json::parse(&text).unwrap();

    let mut inner_map = HashMap::new();
    inner_map.insert(str!("version"), Json::String(str!("1.10.3")));
    let arr = Json::Array(vec![Json::Object(inner_map)]);

    assert_eq!(json,arr);
}

#[test]
fn accepts_empty_object(){
    let text = String::from("{}");
    let json = Json::parse(&text).unwrap();
    assert_eq!(json,Json::Object(HashMap::new()));
}