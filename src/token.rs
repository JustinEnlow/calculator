#[derive(PartialEq, Clone, Copy)]
pub enum Token{
    Number(f32),
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    OpenParen,
    CloseParen,
    Power,
}
impl std::fmt::Debug for Token{
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match *self{
            Token::Number(char) => write!(f, "Number({})", char),
            Token::AddOp => write!(f, "AddOp"),
            Token::SubOp => write!(f, "SubOp"),
            Token::MulOp => write!(f, "MulOp"),
            Token::DivOp => write!(f, "DivOp"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
            Token::Power => write!(f, "Power"),
        }
    }
}
// figure out how to manually implement partial equivalence
//impl std::cmp::PartialEq for Token{
//    fn eq(&self, other: &Self) -> bool {}
//    fn ne(&self, other: &Self) -> bool {}
//}