pub struct JNumberParser;
use crate::parser::common::*;

impl JNumberParser {

    fn number_final_check(text:&str)->bool{
        let r2 = ends_with(text, '.');
        let r3 = starts_with(text, '0') && text.len() > 1;
        return !r2 && !r3;
    }

    fn parse_number_finalize(buffer:String)-> Result<crate::Json,Jerr> {
        if JNumberParser::number_final_check(&buffer) {
            Ok(crate::Json::Number(buffer))
        }
        else{
            Err(Jerr::InvalidToken(buffer))
        }
    }

    fn parse_number_push(iter : &mut StrIt,mut buffer:String,once_dot:&mut bool,c:char)-> Result<Result<crate::Json,Jerr>,String> {
        if is_digit(c) {
            buffer.push(c);
            iter.next();
            Err(buffer)
        }
        else if c == '.' {
            if !*once_dot {
                *once_dot = true;
                buffer.push(c);
                iter.next();
                Err(buffer)
            }
            else{
                Ok(JNumberParser::parse_number_finalize(buffer))
            }
        }
        else{
            Ok(JNumberParser::parse_number_finalize(buffer))
        }
    }

    pub fn parse(iter : &mut StrIt)-> Result<crate::Json,Jerr> {
        let mut buffer = String::new();
        let mut once_dot = false;

        loop {
            match iter.peek().cloned() {
                None => {
                    break JNumberParser::parse_number_finalize(buffer);
                },
                Some((_,c))=>{
                    match JNumberParser::parse_number_push(iter,buffer, &mut once_dot, c) {
                        Err(string)=> buffer = string,
                        Ok(res)=>return res
                    }
                }
            }
        }
    }
}