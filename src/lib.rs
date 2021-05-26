use std::{collections::HashMap, iter::{Enumerate, Peekable}, mem, usize, vec};
pub type StrIt<'a> = Peekable<Enumerate<std::str::Chars<'a>>>;

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
    UnexpectedChar(usize),
    UnexpectedEnd,
    InvalidUnicodeSequence(String),
    UnknownEscape(char),
    ExpectedCommaOrEnd(usize),
    ExpectedColon(usize),
    ExpectedValue(usize),
    ExpectedProperty(usize),
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

    fn push_char(&mut self,c:char)->Result<(),Jerr> {
        if self.is_unicode {
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

    fn parse_string(&mut self,iter:&mut StrIt)->Result<Json,Jerr> {
        iter.next();
        loop {
            if !self.has_ended {
                match iter.next() {
                    None=>break,
                    Some((_,c))=> self.push_char(c)?
                }
            }
            else{
                break
            }
        }
        self.finalize()
    }
}

#[derive(PartialEq,Eq)]
enum ObjectParserState {
    Value,
    Colon,
    Label,
    Comma
}

struct JObjectParser {
    state:ObjectParserState,
    identifier:String,
    map:HashMap<String,Json>
}

impl JObjectParser {
    fn new()->JObjectParser {
        JObjectParser{
            state:ObjectParserState::Label,
            identifier:String::new(),
            map:HashMap::new()
        }
    }

    fn reset(&mut self){
        *self = JObjectParser::new();
    }
    fn push_label(&mut self,iter:&mut StrIt,i:usize)->Result<(),Jerr>{
        match Json::parse(iter) {
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
    fn push_value(&mut self,iter:&mut StrIt,i:usize)->Result<(),Jerr>{
        match Json::parse(iter) {
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
    fn push(&mut self,iter:&mut StrIt,c:char,i:usize)->Result<bool,Jerr>{
        if c == ' ' || c == '\t' || c == '\n' {
            iter.next();
        }
        else {
            match self.state {
                ObjectParserState::Label => self.push_label(iter, i)?,
                ObjectParserState::Colon => self.push_colon(iter, c, i)?,
                ObjectParserState::Value => self.push_value(iter, i)?,
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
                return true 
            }
        }
        false
    }
    fn parse(&mut self,iter:&mut StrIt)->Result<Json,Jerr>{
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
                    if self.push(iter,c,i)? {
                        let map = mem::replace(&mut self.map, HashMap::new());
                        self.reset();
                        return Ok(Json::Object(map));
                    }
                }
            }
        }
    }
}

impl Json {

    fn is_digit(c:char)->bool{
        c >= '0' && c <= '9'
    }

    // all take non-empty strings except parse
    fn is_number(iter:&mut StrIt)->bool{
        Json::is_digit(iter.peek().unwrap().1)
    }

    fn is_string(iter:&mut StrIt)->bool{
        iter.peek().unwrap().1 == '"'
    }

    fn is_array(iter:&mut StrIt)->bool{
        iter.peek().unwrap().1 == '['
    }

    fn is_object(iter:&mut StrIt)->bool{
        iter.peek().unwrap().1 == '{'
    }

    fn starts_with(text:&str,c:char)->bool{
        return text.chars().next().unwrap() == c; 
    }

    fn ends_with(text:&str,c:char)->bool{
        return text.chars().rev().next().unwrap() == c; 
    }

    fn number_final_check(text:&str)->bool{
        let r2 = Json::ends_with(text, '.');
        let r3 = Json::starts_with(text, '0') && text.len() > 1;
        return !r2 && !r3;
    }

    fn parse_number_finalize(buffer:String)-> Result<Json,Jerr> {
        if Json::number_final_check(&buffer) {
            Ok(Json::Number(buffer))
        }
        else{
            Err(Jerr::InvalidToken(buffer))
        }
    }

    fn parse_number(iter : &mut StrIt)-> Result<Json,Jerr> {
        let mut buffer = String::new();
        let mut once_dot = false;

        loop {
            match iter.peek() {
                None => {
                    break Json::parse_number_finalize(buffer);
                },
                Some((_,c))=>{
                    let c = *c;
                    if Json::is_digit(c) {
                        buffer.push(c);
                        iter.next();
                    }
                    else if c == '.' {
                        if !once_dot {
                            once_dot = true;
                            buffer.push(c);
                            iter.next();
                        }
                        else{
                            break Json::parse_number_finalize(buffer);
                        }
                    }
                    else{
                        break Json::parse_number_finalize(buffer);
                    }
                }
            }
        }
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
    fn begins_with_str(iter:&mut StrIt,text:&str)->bool {
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
    fn parse_array(iter:&mut StrIt)->Result<Json,Jerr>{
        iter.next().unwrap();
        let mut vector : Vec<Json> = vec![];
        let mut iterated_value = false;
        if let Some((_,c)) = iter.peek(){
            if *c == ']' {
                return Ok(Json::Array(vec![]));   
            }
        }
        loop {
            match iter.peek() {
                None=>{
                    return Err(Jerr::UnexpectedEnd);
                },
                Some((i,c))=>match *c {
                    ',' | ']' => if iterated_value {
                        let c = *c;
                        iter.next();
                        iterated_value = false;
                        if c == ']' {
                            return Ok(Json::Array(vector));
                        }
                    }
                    else{
                        return Err(Jerr::ExpectedValue(*i));
                    },
                    ' ' | '\n' | '\t'=>{
                        iter.next();
                    },
                    _=> if !iterated_value {
                        vector.push(Json::parse(iter)?);
                        iterated_value = true;
                    }
                    else{
                        return Err(Jerr::ExpectedCommaOrEnd(*i));
                    }
                }
            }
        }
    }
    pub fn parse(iter:&mut StrIt)->Result<Json,Jerr> {
        
        if Json::begins_with_str(iter, "true"){
            return Ok(Json::Bool(true));
        }
        else if Json::begins_with_str(iter, "false"){
            return Ok(Json::Bool(false));
        }
        else if Json::begins_with_str(iter, "null"){
            return Ok(Json::Null);
        }
        else if Json::is_number(iter) {
            Json::parse_number(iter)
        }
        else if Json::is_string(iter) {
            let mut parser = JStringParser::new();
            parser.parse_string(iter)
        }
        else if Json::is_array(iter) {
            Json::parse_array(iter)
        }
        else if Json::is_object(iter) {
            let mut parser = JObjectParser::new();
            parser.parse(iter)
        }
        else { // unknown token
            Err(Jerr::UnexpectedChar(iter.peek().unwrap().0))
        }
    }
}