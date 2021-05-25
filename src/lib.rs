use std::{collections::HashMap, mem};
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
    UnexpectedEnd,
    InvalidUnicodeSequence(String),
    UnknownEscape(char)
}

struct JStringParser {
    has_ended : bool,
    buffer : String,
    escape : bool,
    unicode : String,
    is_unicode : bool
}

impl JStringParser {

    fn new()->JStringParser {
        JStringParser{
            has_ended : false,
            buffer : String::new(),
            escape : false,
            unicode : String::new(),
            is_unicode : false
        }
    }

    fn reset(&mut self){
        *self = JStringParser::new();
    }

    fn push_char_non_escape(&mut self,c:char) {
        if c == '\\' {
            self.escape = true;
        }
        else if c == '"' {
            self.has_ended = true;
        }
        else {
            self.buffer.push(c);
        }
    }


    fn push_char_unicode(&mut self,c:char)->Result<(),Jerr>{
        self.unicode.push(c);
        if self.unicode.len() == 4 {
            let bytes = Json::u8arr_to_u16arr(Json::convert_to_u8(&self.unicode)?);
            self.buffer.push_str(String::from_utf16(bytes.as_ref()).unwrap().as_str());
            self.is_unicode = false;
            self.unicode.clear();
        }
        Ok(())
    }

    fn push_char_escape(&mut self,c:char)->Result<(),Jerr> {
        match c {
            '"' | '\\' => self.buffer.push(c),
            'r' => self.buffer.push('\r'),
            'b' => self.buffer.push('\x08'),
            't' => self.buffer.push('\t'),
            'n' => self.buffer.push('\n'),
            'f' => self.buffer.push('\x0C'),
            'u' => self.is_unicode = true,
            _ => return Err(Jerr::UnknownEscape(c))
        }
        self.escape = false;
        Ok(())
    }

    fn push_char(&mut self,text:&str,c:char)->Result<(),Jerr> {
        if self.has_ended {
            return Err(Jerr::InvalidToken(str!(text)));
        }
        else if self.is_unicode {
            self.push_char_unicode(c)?;
        }
        else if self.escape {
            self.push_char_escape(c)?;
        }
        else{
            self.push_char_non_escape(c);
        }
        Ok(())
    }

    fn finalize(&mut self)->Result<Json,Jerr>{
        if self.has_ended {
            let buff = mem::replace(&mut self.buffer, String::new());
            self.reset();
            Ok(Json::String(buff))
        }
        else{
            self.reset();
            Err(Jerr::UnexpectedEnd)
        }
    }

    fn parse_string(&mut self,text:&str)->Result<Json,Jerr> {
        let mut iter = text.chars();
        iter.next();
        for c in iter {
            self.push_char(text, c)?;
        }
        self.finalize()
    }
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

    fn validate_number(text : &str)-> bool {
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
    fn u8arr_to_u16arr(v:Vec<u8>)->Vec<u16>{
        let mut nv = vec![];
        for i in 0..(v.len()/2) {
            let oc1 = v[i*2];
            let oc2 = v[i*2+1];
            let mut hd : u16 = oc1 as u16;
            hd <<= 8;
            hd += oc2 as u16;
            nv.push(hd)
        }
        return nv;
    }
    fn convert_to_u8(unicode:&String)->Result<Vec<u8>,Jerr>{
        match hex::decode(unicode) {
            Ok(vec)=>Ok(vec),
            Err(_)=>Err(Jerr::InvalidUnicodeSequence(unicode.clone()))
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
                if Json::is_number(trimmed) && Json::validate_number(trimmed){
                    Ok(Json::Number(str!(trimmed)))
                }
                else if Json::is_string(trimmed) {
                    let mut parser = JStringParser::new();
                    parser.parse_string(trimmed)
                }
                else { // unknown token
                    Err(Jerr::InvalidToken(str!(trimmed)))
                }
            }
        }
    }
}