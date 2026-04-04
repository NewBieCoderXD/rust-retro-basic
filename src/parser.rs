use crate::token;

#[derive(Debug)]
pub enum Term {
    Var(String),
    Number(u8),
}

#[derive(Debug)]
pub enum ExpNode {
    Add(Box<ExpNode>, Box<ExpNode>),
    Sub(Box<ExpNode>, Box<ExpNode>),
    Term(Term),
}

#[derive(Debug)]
pub struct CondNode {
    op: token::TokenCompare,
    left: ExpNode,
    right: ExpNode,
}

#[derive(Debug)]
pub enum TreeNode {
    Assign(String, ExpNode),
    If(CondNode, u8),
}
