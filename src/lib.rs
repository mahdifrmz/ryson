use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String,Json>)
}

impl Json {
    pub fn parse(text:&String)->Json {
        if text == "true" {
            Json::Bool(true)
        }
        else if text == "false" {
            Json::Bool(false)
        }
        else{
            Json::Null
        }
    }
}