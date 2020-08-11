#[derive(Debug, PartialEq, Clone)]
pub enum LispValue {
    Atom(String),
    List(Vec<LispValue>),
    DottedList(Vec<LispValue>, Box<LispValue>),
    Number(f64),
    String(String),
    Bool(bool),
}
