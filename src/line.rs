pub enum LineType {
    Plus,
    Minus,
    Times,
    Divide,
    Pow,
    Number,
    Negate,
    Pop,
    UndoPop,
    Clear,
    Print,
    Full,
    ClearScreen,
    Quit,
    Swap,
    Flip,
}
pub struct Line {
    pub line_type: LineType,
    pub value: f64,
    pub combined: bool,
}
