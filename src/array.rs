use std::{mem, usize, vec};

use crate::common::{Json,Jerr,IteratorParser,StrIt};

#[derive(Debug,PartialEq,Eq)]
enum ArrayParserState {
    Beginning,
    Comma,
    Value
}
pub struct JArrayParser {
    vector:Vec<Json>,
    state:ArrayParserState
}

impl JArrayParser {
    pub fn new()->JArrayParser {
        JArrayParser{
            vector:vec![],
            state:ArrayParserState::Beginning
        }
    }
    fn reset(&mut self){
        *self = JArrayParser::new();
    }
    fn push_comma(&mut self,iter:&mut StrIt,i:usize)->Result<(),Jerr>{
        if self.state == ArrayParserState::Comma {
            iter.next();
            self.state = ArrayParserState::Value;
            Ok(())
        }
        else{
            Err(Jerr::ExpectedValue(i))
        }
    }
    fn push_bracket(&mut self,iter:&mut StrIt,i:usize)->Result<Json,Jerr>{
        if self.state == ArrayParserState::Comma || self.state == ArrayParserState::Beginning {
            iter.next();
            let vec = mem::replace(&mut self.vector, vec![]);
            self.reset();
            return Ok(Json::Array(vec));
        }
        else{
            return Err(Jerr::ExpectedValue(i));
        }
    }
    fn push_value(&mut self,iter:&mut StrIt,iterparser:&impl IteratorParser,i:usize)->Result<(),Jerr>{
        if self.state == ArrayParserState::Value || self.state == ArrayParserState::Beginning {
            self.vector.push(iterparser.parse(iter)?);
            self.state = ArrayParserState::Comma;
            Ok(())
        }
        else{
            return Err(Jerr::ExpectedCommaOrEnd(i));
        }
    }
    fn push_space(&mut self,iter:&mut StrIt){
        iter.next();
    }
    pub fn parse(&mut self,iter:&mut StrIt,iterparser:&impl IteratorParser)->Result<Json,Jerr>{
        iter.next().unwrap();
        loop {
            match iter.peek().cloned() {
                None=>{
                    return Err(Jerr::UnexpectedEnd);
                },
                Some((i,c))=>match c {
                    ',' => self.push_comma(iter,i)?,
                    ']' => return self.push_bracket(iter,i),
                    ' ' | '\n' | '\t'=>self.push_space(iter),
                    _=> self.push_value(iter,iterparser, i)?
                }
            }
        }
    }
}