#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    LeftCurly,
    RightCurly,
}

#[derive(Debug, Copy, Clone)]
pub enum Token<'a> {
    Word(&'a str),
    Symbol(Symbol),
}