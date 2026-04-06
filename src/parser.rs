use std::iter::Peekable;

use thiserror::Error;

use crate::{
    terminal::ExpectedNode,
    token::{self, Token},
};

#[derive(Debug)]
pub enum Term {
    Var(String),
    Number(u8),
}

#[derive(Debug)]
pub struct BinaryMathOp {
    pub op: token::TokenMathOp,
    pub left: Box<ExpNode>,
    pub right: Box<ExpNode>,
}

#[derive(Debug)]
pub enum ExpNode {
    BinaryMathOp(BinaryMathOp),
    Term(Term),
}

#[derive(Debug)]
pub struct CondNode {
    pub op: token::TokenCompare,
    pub left: ExpNode,
    pub right: ExpNode,
}

#[derive(Debug)]
pub enum StatementNode {
    Assign(u16, String, ExpNode),
    If(u16, CondNode, u16),
    Goto(u16, u16),
    Print(u16, String),
    Stop(u16),
}

#[derive(Debug)]
pub enum ParseState {
    Start,
    AfterLineNum,
    AssignmentAfterIden,
    AssignmentAfterEqual,
    AfterIf,
    IfAfterCond,
    AfterGoto,
    AfterPrint,
}

#[derive(Error, Debug)]
#[error("Unexpected symbol: `{symbol:?}`, expected `{expected:?}`")]
pub struct ExpectedSymMismatch {
    symbol: Token,
    expected: ExpectedNode,
}

#[derive(Error, Debug)]
#[error("Expected `{expected:?}`")]
pub struct ExpectMismatch {
    expected: ExpectedNode,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    UnexpectedSym(ExpectedSymMismatch),
    #[error("{0}")]
    ExpectMismatch(ExpectMismatch),
    #[error("Expected line number, found {0:?}")]
    ExpectLineNumber(Token),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

fn parse_iden<'a>(
    next_tok: &Token,
    iter: &mut Peekable<impl Iterator<Item = &'a token::Token>>,
) -> Option<String> {
    if let Token::Iden(str) = next_tok {
        iter.next();
        return Some(str.clone());
    }
    return None;
}

fn parse_number<'a>(
    next_tok: &Token,
    iter: &mut Peekable<impl Iterator<Item = &'a token::Token>>,
) -> Option<u16> {
    if let Token::Number(num) = next_tok {
        iter.next();
        return Some(*num);
    }
    return None;
}

fn parse_term<'a>(
    next_tok: &Token,
    iter: &mut Peekable<impl Iterator<Item = &'a token::Token>>,
) -> Option<Term> {
    if let Token::Number(num) = next_tok {
        match u8::try_from(*num) {
            Ok(small_val) => {
                iter.next();
                return Some(Term::Number(small_val));
            }
            Err(_) => return None,
        }
    } else if let Token::Iden(name) = next_tok {
        iter.next();
        return Some(Term::Var((*name).clone()));
    } else {
        return None;
    }
}

fn parse_exp<'a>(
    next_tok: &Token,
    iter: &mut Peekable<impl Iterator<Item = &'a token::Token>>,
) -> Result<Option<ExpNode>,ParseError> {
    let lhs = parse_term(next_tok, iter);
    if let Some(lhs) = lhs {
        let next_tok = iter.peek();
        if let Some(Token::MathOp(math_op)) = next_tok {
            iter.next();

            let next_tok = iter.peek();
            if next_tok.is_none() {
                return Err(ParseError::UnexpectedEndOfInput);
            }
            let next_tok = *next_tok.unwrap();
            let rhs = parse_term(next_tok, iter);
            if let Some(rhs) = rhs {
                return Ok(Some(ExpNode::BinaryMathOp(BinaryMathOp {
                    op: math_op.clone(),
                    left: Box::new(ExpNode::Term(lhs)),
                    right: Box::new(ExpNode::Term(rhs)),
                })));
            } else {
                return Err(ParseError::UnexpectedSym(ExpectedSymMismatch {
                    symbol: next_tok.clone(),
                    expected: ExpectedNode::Expression,
                }));
            }
        } else {
            return Ok(Some(ExpNode::Term(lhs)));
        }
    } else {
        return Ok(None);
    }
}

fn parse_bool_exp<'a>(
    next_tok: &Token,
    iter: &mut Peekable<impl Iterator<Item = &'a token::Token>>,
) -> Result<Option<CondNode>, ParseError> {
    let lhs = parse_term(next_tok, iter);
    if let Some(lhs) = lhs {
        let next_tok = iter.peek();
        if let Some(Token::Compare(comp)) = next_tok {
            iter.next();

            let next_tok = iter.peek();
            if next_tok.is_none() {
                return Err(ParseError::UnexpectedEndOfInput);
            }
            let next_tok = *next_tok.unwrap();
            let rhs = parse_term(next_tok, iter);
            if let Some(rhs) = rhs {
                return Ok(Some(CondNode {
                    op: comp.clone(),
                    left: ExpNode::Term(lhs),
                    right: ExpNode::Term(rhs),
                }));
            } else {
                return Err(ParseError::UnexpectedSym(ExpectedSymMismatch {
                    symbol: next_tok.clone(),
                    expected: ExpectedNode::Expression,
                }));
            }
        } else {
            return Ok(None);
        }
    } else {
        return Ok(None);
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<StatementNode>, ParseError> {
    let mut state = ParseState::Start;
    let mut root = vec![];
    let mut current_line_num: Option<u16> = None;
    let mut iden_name: Option<String> = None;
    let mut cond_node: Option<CondNode> = None;
    let mut iter = tokens.iter().peekable();

    while let Some(&next_tok) = iter.peek() {
        if matches!(next_tok, token::Token::EndOfLine) {
            if !matches!(state, ParseState::Start) {
                return Err(ParseError::UnexpectedSym(ExpectedSymMismatch {
                    symbol: token::Token::EndOfLine,
                    expected: ExpectedNode::Identifier,
                }));
            }
            iter.next();
            continue;
        }
        match state {
            ParseState::Start => {
                if let Some(num) = parse_number(next_tok, &mut iter) {
                    current_line_num = Some(num);
                    state = ParseState::AfterLineNum;
                } else {
                    return Err(ParseError::UnexpectedSym(ExpectedSymMismatch {
                        symbol: (*next_tok).clone(),
                        expected: ExpectedNode::Number,
                    }));
                }
            }
            ParseState::AfterLineNum => {
                if let Some(name) = parse_iden(next_tok, &mut iter) {
                    if let Some(reserved) = token::get_reserved_word(&name) {
                        match reserved {
                            token::ReservedWord::If => {
                                state = ParseState::AfterIf;
                            }
                            token::ReservedWord::Goto => {
                                state = ParseState::AfterGoto;
                            }
                            token::ReservedWord::Print => {
                                state = ParseState::AfterPrint;
                            }
                            token::ReservedWord::Stop => {
                                root.push(StatementNode::Stop(current_line_num.unwrap()));
                                state=ParseState::Start;
                            }
                        }
                    } else {
                        iden_name = Some(name.clone());
                        state = ParseState::AssignmentAfterIden;
                    }
                } else {
                }
            }
            ParseState::AssignmentAfterIden => {
                iter.next();
                if !matches!(next_tok, token::Token::Compare(_)) {
                    return Err(ParseError::UnexpectedSym(ExpectedSymMismatch {
                        symbol: (*next_tok).clone(),
                        expected: ExpectedNode::Equal,
                    }));
                }
                state = ParseState::AssignmentAfterEqual;
            }
            ParseState::AssignmentAfterEqual => {
                let exp = parse_exp(next_tok, &mut iter)?;
                if let Some(exp) = exp {
                    root.push(StatementNode::Assign(
                        current_line_num.unwrap(),
                        iden_name.unwrap(),
                        exp,
                    ));
                    current_line_num = None;
                    iden_name = None;
                    state = ParseState::Start;
                } else {
                    return Err(ParseError::ExpectMismatch(ExpectMismatch {
                        expected: ExpectedNode::Expression,
                    }));
                }
            }
            ParseState::AfterIf => {
                cond_node = parse_bool_exp(next_tok, &mut iter)?;
                if cond_node.is_none() {
                    println!("gg{:?}",next_tok.clone());
                    return Err(ParseError::ExpectMismatch(ExpectMismatch {
                        expected: ExpectedNode::BooleanExpression,
                    }));
                }
                state = ParseState::IfAfterCond;
            }
            ParseState::IfAfterCond => {
                let num = parse_number(next_tok, &mut iter);
                if let Some(num) = num {
                    root.push(StatementNode::If(
                        current_line_num.unwrap(),
                        cond_node.unwrap(),
                        num,
                    ));
                    current_line_num = None;
                    cond_node = None;
                    state = ParseState::Start;
                } else {
                    return Err(ParseError::ExpectLineNumber(next_tok.clone()));
                }
            }
            ParseState::AfterGoto => {
                let num = parse_number(next_tok, &mut iter);
                if let Some(num) = num {
                    root.push(StatementNode::Goto(current_line_num.unwrap(), num));
                    current_line_num = None;
                    state = ParseState::Start;
                    iter.next();
                } else {
                    return Err(ParseError::ExpectLineNumber(next_tok.clone()));
                }
            }
            ParseState::AfterPrint => {
                let iden = parse_iden(next_tok, &mut iter);
                if let Some(iden) = iden {
                    root.push(StatementNode::Print(current_line_num.unwrap(), iden));
                    state = ParseState::Start;
                } else {
                    return Err(ParseError::UnexpectedSym(ExpectedSymMismatch {
                        symbol: next_tok.clone(),
                        expected: ExpectedNode::Identifier,
                    }));
                }
            }
        };
    }
    return Ok(root);
}
