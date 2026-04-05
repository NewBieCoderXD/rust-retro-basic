use thiserror::Error;

use crate::{
    parser::{self, CondNode, ExpNode, StatementNode},
    token,
};

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum BCodeIns {
    LineNum = 10,
    Id = 11,
    Const = 12,
    If = 13,
    Goto = 14,
    Print = 15,
    Stop = 16,
    Op = 17,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum OpCode {
    Add = 1,
    Sub = 2,
    LessThan = 3,
    Equal = 4,
}

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),
    #[error("Unsupported operation: {0:?}")]
    InvalidOp(token::TokenCompare),
}

fn add_line(out: &mut Vec<Vec<u16>>, line: u16) {
    out.push(vec![BCodeIns::LineNum as u16, line]);
}

fn add_iden(last_out: &mut Vec<u16>, iden: String) -> Result<(), CodeGenError> {
    if iden.len() != 1 {
        return Err(CodeGenError::InvalidIdentifier(iden));
    }
    last_out.push(BCodeIns::Id as u16);
    last_out.push(((iden.chars().next().unwrap() as u32) - ('A' as u32) + 1) as u16);
    Ok(())
}

fn add_op(last_out: &mut Vec<u16>, op: OpCode) -> Result<(), CodeGenError> {
    last_out.push(BCodeIns::Op as u16);
    last_out.push(op as u16);
    Ok(())
}

fn add_goto(last_out: &mut Vec<u16>, jump: u16) {
    last_out.push(BCodeIns::Goto as u16);
    last_out.push(jump);
}

fn add_const(last_out: &mut Vec<u16>, val: u8) {
    last_out.push(BCodeIns::Const as u16);
    last_out.push(val as u16);
}

fn add_exp_node(last_out: &mut Vec<u16>, exp: &ExpNode) -> Result<(), CodeGenError> {
    match exp {
        ExpNode::Term(parser::Term::Var(term)) => {
            add_iden(last_out, term.clone())?;
        }
        ExpNode::Term(parser::Term::Number(num)) => {
            add_const(last_out, *num);
        }
        ExpNode::Add(lhs, rhs) => {
            add_exp_node(last_out, lhs.as_ref())?;
            add_op(last_out, OpCode::Add)?;
            add_exp_node(last_out, rhs.as_ref())?;
        }
        ExpNode::Sub(lhs, rhs) => {
            add_exp_node(last_out, lhs.as_ref())?;
            add_op(last_out, OpCode::Sub)?;
            add_exp_node(last_out, rhs.as_ref())?;
        }
    }
    Ok(())
}

// fn add_term(out:&mut Vec<String>, term: &Term){
//   match term{
//     Term::Number(num) => {
//       add_const(out, *num);
//     }
//     Term::Var(var)=>{
//       add_iden(out, var.clone());
//     }
//   }
// }

fn add_cond_node(last_out: &mut Vec<u16>, cond: &CondNode) -> Result<(), CodeGenError> {
    let op;
    match cond.op {
        token::TokenCompare::Equal => op = OpCode::Equal,
        token::TokenCompare::MoreThan => {
            return Err(CodeGenError::InvalidOp(token::TokenCompare::MoreThan));
        }
        token::TokenCompare::LessThan => op = OpCode::LessThan,
    };
    add_exp_node(last_out, &cond.left)?;
    add_op(last_out, op)?;
    add_exp_node(last_out, &cond.right)?;
    Ok(())
}

pub fn generate(tree: Vec<StatementNode>) -> Result<String, CodeGenError> {
    let mut out = vec![];
    for stmt in tree {
        match stmt {
            StatementNode::Assign(line, iden, val) => {
                add_line(&mut out, line);
                let last_out = out.last_mut().unwrap();
                add_iden(last_out, iden)?;
                add_op(last_out, OpCode::Equal)?;
                add_exp_node(last_out, &val)?
            }
            StatementNode::If(line, cond, jump) => {
                add_line(&mut out, line);
                let last_out = out.last_mut().unwrap();
                last_out.push(BCodeIns::If as u16);
                last_out.push(0);
                add_cond_node(last_out, &cond)?;
                add_goto(last_out, jump);
            }
            StatementNode::Goto(line, jump) => {
                add_line(&mut out, line);
                let last_out = out.last_mut().unwrap();
                add_goto(last_out, jump);
            }
            StatementNode::Print(line, var) => {
                add_line(&mut out, line);
                let last_out = out.last_mut().unwrap();
                last_out.push(BCodeIns::Print as u16);
                last_out.push(0);
                add_iden(last_out, var)?;
            }
            StatementNode::Stop(line) => {
                add_line(&mut out, line);
                let last_out = out.last_mut().unwrap();
                last_out.push(BCodeIns::Stop as u16);
                last_out.push(0);
            }
        }
    }
    out.push(vec![0]);
    Ok(out
        .iter()
        .map(|line| {
            line.iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n"))
}
