mod parser;
mod display;
pub use parser::Jerr;

#[derive(PartialEq,Eq,Clone)]
pub enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(parser::Jmap)
}