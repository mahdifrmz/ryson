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

    // all take non-empty strings except parse
    fn is_number(text:&str)->bool{
        Json::is_digit(text.chars().next().unwrap())
    }

    fn is_string(text:&str)->bool{
        text.chars().next().unwrap() == '"'
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
    fn try_parse_string(text:&str)->Result<(),Jerr> {
        let mut has_ended = false;
        let mut iter = text.chars();
        iter.next();
        for c in iter {
            println!("char:{}",c);
            if has_ended {
                return Err(Jerr::InvalidToken(str!(text)));
            }
            if c == '"' {
                has_ended = true;
            }
        }
        if has_ended {
            Ok(())
        }
        else{
            Err(Jerr::UnexpectedEnd)
        }
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
                else if Json::is_string(trimmed) {
                    match Json::try_parse_string(trimmed) {
                        Err(jerr)=>Err(jerr),
                        Ok(())=>Ok(Json::String(str!(trimmed)))
                    }
                }
                else { // unknown token
                    Err(Jerr::InvalidToken(str!(trimmed)))
                }
            }            
        }
    }
}