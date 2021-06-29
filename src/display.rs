use crate::Json;

impl Json {
    pub fn to_string(&self)->String {
        match self {
            Json::Null => {
                String::from("null")
            },
            Json::Number(num) => {
                num.clone()
            },
            Json::String(str) => {
                format!("\"{}\"",str)
            },
            Json::Bool(bl) => {
                if *bl {
                    String::from("true")
                }
                else{
                    String::from("false")
                }
            },
            _ => {
                String::from("")
            },
        }
    }
}