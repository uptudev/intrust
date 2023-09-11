#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let {name: String, value: Box<Expression>},     // mutable
    Const {name: String, value: Box<Expression>},   // immutable
    Assignment {name: String, value: Box<Expression>},
    While {
        condition: Box<Expression::Condition>,
        body: Vec<Statement>,
    },
    Print {value: Box<Expression>},
    Return {value: Box<Expression>},
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Condition(bool),
    Variable(String),
    Int(i32),
    Long(i64),
    UInt(u32),
    ULong(u64),
    Float(f32),
    Double(f64),
    NumOp { lhs: Box<Expression>, operator: Operator, rhs: Box<Expression>},
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    
    // Binary Operators
    AND,        // &
    OR,         // |
    NOT,        // !
    XOR,        // ^
    NAND,       // !&
    NOR,        // !|
    NXOR,       // !^
    LShift,     // <<
    RShift,     // >>

    // Comparison Operators
    Eq,         // ==
    NotEq,      // !=
    LessThan,   // <
    GreatThan,  // >
    LTEq,       // <=
    GTEq,       // >=
                
    // Comparison Logic Operators
    LogicAND,   // &&
    LogicOR,    // ||
    LogicXOR,   // ^^
    LogicNAND,  // !&&
    LogicNOR,   // !||
    LogicXNOR,  // !^^
}

