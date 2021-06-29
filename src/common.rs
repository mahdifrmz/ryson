use std::{collections::HashMap, iter::{Enumerate, Peekable}};
pub type StrIt<'a> = Peekable<Enumerate<std::str::Chars<'a>>>;
pub type Jmap = HashMap<String,Json>;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(Jmap)
}

#[derive(Debug,PartialEq,Eq)]
pub enum Jerr {
    InvalidToken(String),
    UnexpectedChar(usize),
    UnexpectedEnd,
    InvalidUnicodeSequence(String),
    UnknownEscape(char),
    ExpectedCommaOrEnd(usize),
    ExpectedColon(usize),
    ExpectedValue(usize),
    ExpectedProperty(usize),
    ExpectedEnd(usize)
}

pub fn is_digit(c:char)->bool{
    c >= '0' && c <= '9'
}

// all take non-empty strings except parse
pub fn is_number(iter:&mut StrIt)->bool{
    is_digit(iter.peek().unwrap().1)
}

pub fn is_string(iter:&mut StrIt)->bool{
    iter.peek().unwrap().1 == '"'
}

pub fn is_array(iter:&mut StrIt)->bool{
    iter.peek().unwrap().1 == '['
}

pub fn is_object(iter:&mut StrIt)->bool{
    iter.peek().unwrap().1 == '{'
}

pub fn starts_with(text:&str,c:char)->bool{
    return text.chars().next().unwrap() == c; 
}

pub fn ends_with(text:&str,c:char)->bool{
    return text.chars().rev().next().unwrap() == c; 
}

pub fn u8arr_to_u16arr(v:Vec<u8>)->Vec<u16>{
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

pub fn convert_to_u8(unicode:&String)->Result<Vec<u8>,Jerr>{
    match hex::decode(unicode) {
        Ok(vec)=>Ok(vec),
        Err(_)=>Err(Jerr::InvalidUnicodeSequence(unicode.clone()))
    }
}

pub fn begins_with_str(iter:&mut StrIt,text:&str)->bool {
    let mut it = iter.clone();
    for c in text.chars() {
        match it.next() {
            Some(ic)=>{
                if ic.1 != c {
                    return false;
                }
            },
            None=>{
                return false;
            }
        }
    }
    *iter = it;
    true

}

pub fn make_iterator(text:&str)->StrIt{
    text.chars().enumerate().peekable()
}

pub trait IteratorParser {
    fn parse (&self,iter: &mut StrIt)->Result<Json,Jerr>;
}