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