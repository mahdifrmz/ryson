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
            Json::Array(vec)=>{
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
            _ => {
                String::from("")
            },
        }
    }
}