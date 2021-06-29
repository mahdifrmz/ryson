use std::{collections::HashMap, mem};
use crate::common::*;

#[derive(PartialEq,Eq)]
enum ObjectParserState {
    Value,
    Colon,
    Label,
    Comma
}

pub struct JObjectParser {
    state:ObjectParserState,
    identifier:String,
    map:Jmap
}

impl JObjectParser {
    pub fn new()->JObjectParser {
        JObjectParser{
            state:ObjectParserState::Label,
            identifier:String::new(),
            map:HashMap::new()
        }
    }

    fn reset(&mut self){
        *self = JObjectParser::new();
    }
    fn push_label(&mut self,iter:&mut StrIt,i:usize,iterparser:&impl IteratorParser)->Result<(),Jerr>{
        match iterparser.parse(iter) {
            Ok(json)=>{
                match json {
                    Json::String(str)=>{
                        self.identifier = str;
                        self.state = ObjectParserState::Colon;
                        Ok(())
                    },
                    _=>{
                        Err(Jerr::ExpectedProperty(i))
                    }
                }
            }
            Err(_) => Err(Jerr::ExpectedProperty(i))
        }
    }
    fn push_colon(&mut self,iter:&mut StrIt,c:char,i:usize)->Result<(),Jerr>{
        match c {
            ':' => {
                iter.next();
                self.state = ObjectParserState::Value;
                Ok(())
            },
            _=>{
                Err(Jerr::ExpectedColon(i))
            }
        }
    }
    fn push_value(&mut self,iter:&mut StrIt,i:usize,iterparser:&impl IteratorParser)->Result<(),Jerr>{
        match iterparser.parse(iter) {
            Ok(json)=>{
                self.map.insert(mem::replace(&mut self.identifier, String::new()),json);
                self.state = ObjectParserState::Comma;
                Ok(())
            }
            Err(_) => Err(Jerr::ExpectedValue(i))
        }
    }
    fn push_comma(&mut self,iter:&mut StrIt,c:char,i:usize)->Result<bool,Jerr>{
        match c {
            ',' | '}' => {
                iter.next();
                self.state = ObjectParserState::Label;
                Ok(c == '}')
            },
            _=>{
                Err(Jerr::ExpectedCommaOrEnd(i))
            }
        }
    }
    fn push(&mut self,iter:&mut StrIt,c:char,i:usize,iterparser:&impl IteratorParser)->Result<bool,Jerr>{
        if c == ' ' || c == '\t' || c == '\n' {
            iter.next();
        }
        else {
            match self.state {
                ObjectParserState::Label => self.push_label(iter, i,iterparser)?,
                ObjectParserState::Colon => self.push_colon(iter, c, i)?,
                ObjectParserState::Value => self.push_value(iter, i,iterparser)?,
                ObjectParserState::Comma => {
                    if self.push_comma(iter, c, i)? {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
    fn init_check(&self,iter:&mut StrIt)->bool{
        if let Some((_,c)) = iter.peek(){
            if *c == '}' {
                iter.next();
                return true 
            }
        }
        false
    }
    pub fn parse(&mut self,iter:&mut StrIt,iterparser:&impl IteratorParser)->Result<Json,Jerr>{
        iter.next().unwrap();
        if self.init_check(iter) {
            return Ok(Json::Object(HashMap::new()));
        }
        loop {
            match iter.peek() {
                None=>{
                    return Err(Jerr::UnexpectedEnd);
                },
                Some((i,c))=> {
                    let c = *c;
                    let i = *i;
                    if self.push(iter,c,i,iterparser)? {
                        let map = mem::replace(&mut self.map, HashMap::new());
                        self.reset();
                        return Ok(Json::Object(map));
                    }
                }
            }
        }
    }
}