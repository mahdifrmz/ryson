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

    // takes non-empty string
    fn is_number(text:&str)->bool{
        Json::is_digit(text.chars().next().unwrap())
    }

    fn starts_with(text:&str,c:char)->bool{
        return text.chars().next().unwrap() == c; 
    }

    fn ends_with(text:&str,c:char)->bool{
        return text.chars().rev().next().unwrap() == c; 
    }

    fn number_initial_check(text:&str)->bool{
        let r1 = Json::starts_with(text, '.');
        let r2 = Json::ends_with(text, '.');
        let r3 = Json::starts_with(text, '0') && text.len() > 1;
        return !r1 && !r2 && !r3;
    }

    fn try_parse_number(text : &str)-> bool {
        if !Json::number_initial_check(text) {
            return false;
        }
        let mut once_dot = false;
        for c in text.chars() {
            if !Json::is_digit(c) && c != '.'{
                return false
            }
            else if c == '.' {
                if once_dot {
                    return false;
                }
                once_dot = true;
            }
        }
        true
    }
    pub fn parse(text:&String)->Result<Json,Jerr> {
        let trimmed = text.trim();
        match trimmed {
            ""=>Err(Jerr::UnexpectedEnd),
            "true"=>Ok(Json::Bool(true)),
            "false"=>Ok(Json::Bool(false)),
            "null"=>Ok(Json::Null),
            _=>{
                if Json::is_number(trimmed) && Json::try_parse_number(trimmed){
                    Ok(Json::Number(str!(trimmed)))
                }
                else { // unknown token
                    Err(Jerr::InvalidToken(str!(trimmed)))
                }
            }            
        }
    }
}