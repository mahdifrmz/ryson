use std::mem;

use crate::parser::common::*;
pub struct JStringParser {
    has_ended : bool,
    buffer : String,
    escape : bool,
    unicode : String,
    is_unicode : bool
}

impl JStringParser {

    pub fn new()->JStringParser {
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
            let bytes = u8arr_to_u16arr(convert_to_u8(&self.unicode)?);
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

    fn finalize(&mut self)->Result<crate::Json,Jerr>{
        if self.has_ended {
            let buff = mem::replace(&mut self.buffer, String::new());
            self.reset();
            Ok(crate::Json::String(buff))
        }
        else{
            self.reset();
            Err(Jerr::UnexpectedEnd)
        }
    }

    pub fn parse(&mut self,iter:&mut StrIt)->Result<crate::Json,Jerr> {
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