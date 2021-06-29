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
            },
            Json::Object(map)=>{
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
        }
    }
}