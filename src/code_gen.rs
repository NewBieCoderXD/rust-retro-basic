use thiserror::Error;

use crate::{parser::{self, CondNode, ExpNode, StatementNode}, token};

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

fn add_line(out: &mut Vec<String>, line: u16) {
    out.push((BCodeIns::LineNum as u8).to_string());
    out.push(line.to_string());
}

fn add_iden(out: &mut Vec<String>, iden: String) -> Result<(), CodeGenError> {
    if iden.len() != 1 {
        return Err(CodeGenError::InvalidIdentifier(iden));
    }
    out.push((BCodeIns::Id as u8).to_string());
    out.push(((iden.chars().next().unwrap() as u32) - ('A' as u32)+1).to_string());
    Ok(())
}

fn add_op(out: &mut Vec<String>, op: OpCode) -> Result<(), CodeGenError> {
    out.push((BCodeIns::Op as u8).to_string());
    out.push((op as u8).to_string());
    Ok(())
}

fn add_goto(out: &mut Vec<String>, jump: u16) {
    out.push((BCodeIns::Goto as u8).to_string());
    out.push(jump.to_string());
}

fn add_const(out: &mut Vec<String>, val: u8){
  out.push((BCodeIns::Const as u8).to_string());
  out.push(val.to_string());
}

fn add_exp_node(out: &mut Vec<String>, exp: &ExpNode) -> Result<(),CodeGenError>{
  match exp{
    ExpNode::Term(parser::Term::Var(term)) => {
      add_iden(out, term.clone())?;
    }
    ExpNode::Term(parser::Term::Number(num)) => {
      add_const(out, *num);
    }
    ExpNode::Add(lhs, rhs) => {
      add_exp_node(out, lhs.as_ref())?;
      add_op(out, OpCode::Add)?;
      add_exp_node(out, rhs.as_ref())?;
    }
    ExpNode::Sub(lhs, rhs) => {
      add_exp_node(out, lhs.as_ref())?;
      add_op(out, OpCode::Sub)?;
      add_exp_node(out, rhs.as_ref())?;
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

fn add_cond_node(out: &mut Vec<String>, cond: &CondNode) -> Result<(),CodeGenError>{
    let op;
    match cond.op{
      token::TokenCompare::Equal => {op=OpCode::Equal}
      token::TokenCompare::MoreThan => {return Err(CodeGenError::InvalidOp(token::TokenCompare::MoreThan))}
      token::TokenCompare::LessThan => {op=OpCode::LessThan}
    };
      add_exp_node(out, &cond.left)?;
      add_op(out, op)?;
      add_exp_node(out, &cond.right)?;
      Ok(())
}

pub fn generate(tree: Vec<StatementNode>) -> Result<String, CodeGenError> {
    let mut out = vec![];
    for stmt in tree {
        match stmt {
            StatementNode::Assign(line, iden, val) => {
                add_line(&mut out, line);
                add_iden(&mut out, iden)?;
                add_op(&mut out, OpCode::Equal)?;
                add_exp_node(&mut out, &val)?
            }
            StatementNode::If(line, cond, jump) => {
                add_line(&mut out, line);
                out.push((BCodeIns::If as u8).to_string());
                add_cond_node(&mut out, &cond)?;
                add_goto(&mut out, jump);
            }
            StatementNode::Goto(line, jump) => {
                add_line(&mut out, line);
                add_goto(&mut out, jump);
            }
            StatementNode::Print(line, var) => {
                add_line(&mut out, line);
                out.push((BCodeIns::Print as u8).to_string());
                out.push(var)
            }
            StatementNode::Stop(line) => {
                add_line(&mut out, line);
                out.push((BCodeIns::Stop as u8).to_string());
            }
        }
    }
    out.push("0".to_owned());
    Ok(out.join(" "))
}
