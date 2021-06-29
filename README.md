# Ryson

Ryson is a library for extracting json values from strings.
the library's API provides a single **parse** function which
can be used to get the json enum from a string:

```
enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String,Json>)
}
```

to use the library simply add the following to the **dependency**
section of your Cargo.toml:

```
ryson = "1.0.0"
```

then simply parse any json string:

```
let text : String = String::from("{\"port\":80}");
let config : Json = ryson::Json::parse(&text).unwrap();
```

the above will give you a Json enum that can be then matched against all json types.
to convert that to a map and extract the port number:
```
let map : &Hashmap<String,Json> = config.as_object().unwrap();
let port : &Json = map.get(&String::from("port")).unwrap();
let port_number : String = port.as_number().unwrap().clone();
println!("port={}",port_number);
```
Note that the json numbers are represented by strings in this library, as there is no built-in
feature in Rust for storing big JS numbers.