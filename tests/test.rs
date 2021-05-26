use ryson::{Jerr, Json, StrIt};
use str_macro::str;

fn make_iterator(text:&str)->StrIt{
    text.chars().enumerate().peekable()
}

#[test]
fn accepts_null(){
    let mut text = make_iterator("null");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Null);
}

#[test]
fn accepts_true(){
    let mut text = make_iterator("true");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Bool(true));
}

#[test]
fn accepts_false(){
    let mut text = make_iterator("false");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Bool(false));
}

#[test]
fn throws_error_on_unknown_keyword(){
    let mut text = make_iterator("True");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedChar(0));
}

#[test]
fn accepts_integers(){
    let mut text = make_iterator("1024");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Number(str!("1024")));
}

#[test]
fn error_on_non_digits_after_digits(){
    let mut text = make_iterator("4534h");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Number(str!("4534")));
}

#[test]
fn error_on_non_zero_starting_with_zero(){
    let mut text = make_iterator("0916");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!("0916")));
}

#[test]
fn accepts_rationals(){
    let mut text = make_iterator("16.824");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Number(str!("16.824")));
}

#[test]
fn error_on_ending_dot(){
    let mut text = make_iterator("1624.");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidToken(str!("1624.")));
}


#[test]
fn error_on_beginning_dot(){
    let mut text = make_iterator(".234567");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedChar(0));
}

#[test]
fn error_on_multiple_dots(){
    let mut text = make_iterator("23.456.7");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Number(str!("23.456")));
}

#[test]
fn accepts_strings(){
    let mut text = make_iterator("\"hello world\"");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::String(str!("hello world")));
}

#[test]
fn unexpected_end_of_string(){
    let mut text = make_iterator("\"hello world");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn error_on_text_after_ending_quote(){
    let mut text = make_iterator("\"hello \nworld");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn escapes_back_slash_quote(){
    let mut text = make_iterator("\"a quote is a \\\" sign\"");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::String(str!("a quote is a \" sign")));
}

#[test]
fn escapes_double_back_slash(){
    let mut text = make_iterator("\"a backslash is a \\\\ sign\"");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::String(str!("a backslash is a \\ sign")));
}

#[test]
fn escapes_criagereturn_tab_newline_formfeed_backspace(){
    let mut text = make_iterator("\"escaped:\\n\\thello\\b\\ftext file\\r\"");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::String(str!("escaped:\n\thello\x08\x0Ctext file\r")));
}

#[test]
fn escapes_unicode(){
    let mut text = make_iterator("\"this is theta : \\u03F4\"");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::String(str!("this is theta : ϴ")));
}

#[test]
fn error_on_invalid_unicode(){
    let mut text = make_iterator("\"this is invalid : \\u93G4\"");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::InvalidUnicodeSequence(str!("93G4")));
}

#[test]
fn error_on_unknown_escape(){
    let mut text = make_iterator("\"I don't know \\a\"");
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::UnknownEscape('a'));
}

#[test]
fn iterator_preserves_position(){
    let mut text = make_iterator("null,");
    Json::parse(&mut text).unwrap();
    assert_eq!(text.peek().unwrap().0,4);
}

#[test]
fn preserves_position_on_number(){
    let mut text = make_iterator("234 ");
    Json::parse(&mut text).unwrap();
    assert_eq!(text.peek().unwrap().0,3);
}

#[test]
fn preserves_position_on_string(){
    let mut text = make_iterator("\"text\":true");
    Json::parse(&mut text).unwrap();
    assert_eq!(text.peek().unwrap().0,6);
}

#[test]
fn single_element_array(){
    let mut text = make_iterator("[false]");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Array(vec![Json::Bool(false)]));
}

#[test]
fn multi_element_array(){
    let mut text = make_iterator(
        "[true,1444,\"third element\"]"
    );
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Array(vec![
        Json::Bool(true),
        Json::Number(str!("1444")),
        Json::String(str!("third element"))
    ]));
}

#[test]
fn ignore_white_space_newline(){
    let mut text = make_iterator(
        "[true,  1444\n,  \"third element\"\n\n  ]"
    );
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Array(vec![
        Json::Bool(true),
        Json::Number(str!("1444")),
        Json::String(str!("third element"))
    ]));
}

#[test]
fn error_on_not_ended_array(){
    let mut text = make_iterator(
        "[true,  1444\n,  \"third element\"\n\n  "
    );
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::UnexpectedEnd);
}

#[test]
fn error_on_multiple_commas(){
    let mut text = make_iterator(
        "[true,  1444\n, , \"third element\"\n\n  "
    );
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedValue(15));
}

#[test]
fn error_on_multiple_value(){
    let mut text = make_iterator(
        "[true,  1444\n \"third element\"\n\n  "
    );
    let jerr = Json::parse(&mut text).unwrap_err();
    assert_eq!(jerr,Jerr::ExpectedCommaOrEnd(14));
}

#[test]
fn accept_nested_arrays(){
    let mut text = make_iterator("[\n   [false]\n]");
    let json = Json::parse(&mut text).unwrap();
    assert_eq!(json,Json::Array(vec![Json::Array(vec![Json::Bool(false)])]));
}