use std::collections::HashMap;
use str_macro::str;

#[derive(Debug,PartialEq,Eq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String,Json>)
}

#[derive(Debug,PartialEq,Eq)]
pub enum Jerr {
    InvalidToken(String),
    UnexpectedEnd
}

impl Json {

    fn is_digit(c:char)->bool{
        c >= '0' && c <= '9'
    }

    fn try_parse_number(text : &str)-> Result<bool,Jerr> {
        if text.len() > 1 && text.chars().next().unwrap() == '0' {
            return Err(Jerr::InvalidToken(String::from(text)));
        }
        for c in text.chars() {
            if !Json::is_digit(c){
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub fn parse(text:&String)->Result<Json,Jerr> {
        let trimmed = text.trim();
        if trimmed == "true" {
            Ok(Json::Bool(true))
        }
        else if trimmed == "false" {
            Ok(Json::Bool(false))
        }
        else if trimmed == "null"{
            Ok(Json::Null)
        }
        else if trimmed == ""{
            Err(Jerr::UnexpectedEnd)
        }
        else if Json::try_parse_number(trimmed)?{
            Ok(Json::Number(String::from(trimmed)))
        }
        else {
            Err(Jerr::InvalidToken(str!(trimmed)))
        }
    }
}