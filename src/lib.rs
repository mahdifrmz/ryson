mod parser;
mod display;
pub use parser::Jerr;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(parser::Jmap)
}