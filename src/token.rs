use strum_macros::{Display, EnumString};

#[derive(Debug,Clone)]
pub enum TokenCompare {
    Equal,
    LessThan,
    MoreThan,
}

#[derive(Debug,Clone)]
pub enum TokenMathOp{
  Add,
  Sub
}

#[derive(Debug,Clone)]
pub enum Token {
    MathOp(TokenMathOp),
    Number(u16),
    Iden(String),
    Compare(TokenCompare),
    EndOfLine
}

#[derive(Debug, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ReservedWord {
    If,
    Goto,
    Print,
    Stop
}

pub fn get_reserved_word(input: &str) -> Option<ReservedWord> {
    let word = ReservedWord::try_from(input);

    if let Ok(word) = word {
        return Some(word);
    } else {
      return None;
    }
}
