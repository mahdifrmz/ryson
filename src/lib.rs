mod common;
mod object;
mod string;
mod array;
mod number;

use common::*;
pub use common::{Json,Jerr};

struct BasicIteratorParser;

impl IteratorParser for BasicIteratorParser {
    fn parse(&self,iter:&mut StrIt)->Result<Json,Jerr> {
        
        if begins_with_str(iter, "true"){
            return Ok(Json::Bool(true));
        }
        else if begins_with_str(iter, "false"){
            return Ok(Json::Bool(false));
        }
        else if begins_with_str(iter, "null"){
            return Ok(Json::Null);
        }
        else if is_number(iter) {
            number::JNumberParser::parse(iter)
        }
        else if is_string(iter) {
            let mut parser = crate::string::JStringParser::new();
            parser.parse(iter)
        }
        else if is_array(iter) {
            let mut parser = crate::array::JArrayParser::new();
            parser.parse(iter,self)
        }
        else if is_object(iter) {
            let mut parser = crate::object::JObjectParser::new();
            parser.parse(iter,self)
        }
        else { // unknown token
            Err(Jerr::UnexpectedChar(iter.peek().unwrap().0))
        }
    }
}

impl Json {
    pub fn parse(input:&String)->Result<Json,Jerr> {
        let mut iter = make_iterator(input.as_str().trim());
        let basic_parser = BasicIteratorParser;
        let json = basic_parser.parse(&mut iter)?;
        match iter.peek()  {
            None=>Ok(json),
            Some((i,_))=>Err(Jerr::ExpectedEnd(*i))
        }
    }
    pub fn as_number(&self)-> Option<&String> {
        match self {
            Json::Number(num)=>Some(num),
            _=>None
        }
    }
    pub fn as_string(&self)-> Option<&String> {
        match self {
            Json::String(str)=>Some(str),
            _=>None
        }
    }
    pub fn as_bool(&self)-> Option<&bool> {
        match self {
            Json::Bool(b)=>Some(b),
            _=>None
        }
    }
    pub fn as_null(&self)-> Option<()> {
        match self {
            Json::Null=>Some(()),
            _=>None
        }
    }
    pub fn as_array(&self)-> Option<&Vec<Json>> {
        match self {
            Json::Array(vec)=>Some(vec),
            _=>None
        }
    }
    pub fn as_mut_array(&mut self)-> Option<&mut Vec<Json>> {
        match self {
            Json::Array(vec)=>Some(vec),
            _=>None
        }
    }
    pub fn as_object(&self)-> Option<&Jmap> {
        match self {
            Json::Object(o)=>Some(o),
            _=>None
        }
    }
    pub fn as_mut_object(&mut self)-> Option<&mut Jmap> {
        match self {
            Json::Object(o)=>Some(o),
            _=>None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{BasicIteratorParser, common::{IteratorParser, make_iterator}};

    #[test]
    fn iterator_preserves_position(){
        let text = String::from("null,");
        let mut iter = make_iterator(text.as_str());
        let parser = BasicIteratorParser;
        parser.parse(&mut iter).unwrap();
        assert_eq!(iter.peek().unwrap().0,4);
    }

    #[test]
    fn preserves_position_on_number(){
        let text = String::from("234 ");
        let mut iter = make_iterator(text.as_str());
        let parser = BasicIteratorParser;
        parser.parse(&mut iter).unwrap();
        assert_eq!(iter.peek().unwrap().0,3);
    }

    #[test]
    fn preserves_position_on_string(){
        let text = String::from("\"text\":true");
        let mut iter = make_iterator(text.as_str());
        let parser = BasicIteratorParser;
        parser.parse(&mut iter).unwrap();
        assert_eq!(iter.peek().unwrap().0,6);
    }
}