use ryson::Json;

#[test]
fn accepts_null(){
    let text = String::from("null");
    let json = Json::parse(&text);
    assert_eq!(json,Json::Null);
}

#[test]
fn accepts_true(){
    let text = String::from("true");
    let json = Json::parse(&text);
    assert_eq!(json,Json::Bool(true));
}


#[test]
fn accepts_false(){
    let text = String::from("false");
    let json = Json::parse(&text);
    assert_eq!(json,Json::Bool(false));
}