use std::{collections::HashMap, fmt::{self, Debug, Display}};

use crate::Json;

fn array_to_string(vec:&Vec<Json>)-> String {
    let mut text = String::from("[");
    for json in vec {
        text.push_str(json.to_string().as_str());
        text.push(',');
    }
    if text.chars().last().unwrap() == ',' {
        text.pop();
    }
    text.push(']');
    text
}

fn object_to_string(map:&HashMap<String,Json>)-> String {
    let mut text = String::from("{");
    for key in map.keys() {
        let value = map.get(key).unwrap();
        text.push_str(format!("{}:{},",key,value.to_string()).as_str());
    }
    if text.chars().last().unwrap() == ',' {
        text.pop();
    }
    text.push('}');
    text
}

impl Json {
    fn to_string(&self)->String {
        match self {
            Json::Null => String::from("null"),
            Json::Number(num) => num.clone(),
            Json::String(str) => format!("\"{}\"",str),
            Json::Bool(bl) => if *bl {
                String::from("true")
            } else {
                String::from("false")
            },
            Json::Array(vec)=>array_to_string(vec),
            Json::Object(map)=>object_to_string(map)
        }
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}