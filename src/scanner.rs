use std::path::PathBuf;

use tokio::io::AsyncReadExt;
use thiserror::Error;
use tokio::fs::File;
use crate::{constants::SCAN_BUF_SIZE, token};

#[derive(Debug)]
pub enum ScanState {
    Start,
    StartIdentifier,
    StartNumber,
}

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Invalid Operation: {0}")]
    UnsupportedOp(String),
    #[error("Invalid Character for Identifier: {0}")]
    UnsupportedCharForIden(char),
    #[error("Identifier cannot start with a number")]
    IdenStartWithNum,
    #[error("Error when reading file: {0}")]
    CannotReadFile(std::io::Error),
}

impl From<std::io::Error> for ScanError {
    fn from(value: std::io::Error) -> Self {
        ScanError::CannotReadFile(value)
    }
}



pub fn on_finish_term(state: &mut ScanState, mem: &mut Vec<char>, out: &mut Vec<token::Token>) {
    match state {
        ScanState::StartNumber => {
            let str: String = mem.iter().collect();
            let num: u16 = str.parse().unwrap();
            out.push(token::Token::Number(num));
            *state = ScanState::Start;
            mem.clear();
        }
        ScanState::StartIdentifier => {
            out.push(token::Token::Iden(mem.iter().collect()));
            *state = ScanState::Start;
            mem.clear();
        }
        _ => {}
    };
}

pub fn scan(
    raw_chars: &Vec<u8>,
    current_size: usize,
    state: &mut ScanState,
    mem: &mut Vec<char>,
) -> Result<Vec<token::Token>, ScanError> {
    let mut out = vec![];
    let chars: Vec<char> = raw_chars.iter().map(|&num| num as char).collect();
    for &char in chars.iter().take(current_size) {
        if char == '\r' {
            continue;
        }
        if char == ' ' || char == '\n' {
            on_finish_term(state, mem, &mut out);
            if char == '\n' {
                out.push(token::Token::EndOfLine);
            }
            continue;
        }
        match state {
            ScanState::Start => {
                if char.is_ascii_digit() {
                    *state = ScanState::StartNumber;
                    mem.push(char);
                } else if char.is_ascii_alphabetic() {
                    *state = ScanState::StartIdentifier;
                    mem.push(char);
                } else if char == '=' || char == '<' {
                    let compare = if char == '=' {
                        token::TokenCompare::Equal
                    } else {
                        token::TokenCompare::LessThan
                    };
                    out.push(token::Token::Compare(compare));
                    *state = ScanState::Start;
                    mem.clear();
                } else if char == '+' || char == '-' {
                    let math_op = if char == '+' {
                        token::TokenMathOp::Add
                    } else {
                        token::TokenMathOp::Sub
                    };
                    out.push(token::Token::MathOp(math_op));
                    *state = ScanState::Start;
                    mem.clear();
                } else {
                    return Err(ScanError::UnsupportedOp(char.to_string()));
                }
            }
            ScanState::StartIdentifier => {
                if char.is_ascii_alphanumeric() {
                    mem.push(char);
                } else {
                    return Err(ScanError::UnsupportedCharForIden(char));
                }
            }
            ScanState::StartNumber => {
                if char.is_ascii_digit() {
                    mem.push(char);
                } else {
                    return Err(ScanError::IdenStartWithNum);
                }
            }
        }
    }
    Ok(out)
}


pub async fn process_buffers_and_scan(
    path: PathBuf,
    state: &mut ScanState,
    mem: &mut Vec<char>,
) -> Result<Vec<token::Token>, ScanError> {
    let mut file = File::open(path).await?;

    let mut buf_a = vec![0u8; SCAN_BUF_SIZE];
    let mut buf_b = vec![0u8; SCAN_BUF_SIZE];

    let n = file.read(&mut buf_a).await?;
    if n == 0 {
        return Ok(vec![]);
    }

    let mut current_size = n;

    let mut current_is_a = true;
    let mut real_out = vec![];

    loop {
        if current_size == 0 {
            break;
        }

        let active_buf;
        let next_read_result;
        if current_is_a {
            next_read_result = file.read(&mut buf_b);
            active_buf = &buf_a;
        } else {
            next_read_result = file.read(&mut buf_a);
            active_buf = &buf_b;
        };

        let out = scan(active_buf, current_size, state, mem)?;
        real_out.extend(out);

        match next_read_result.await {
            Ok(new_size) => {
                current_is_a = !current_is_a;
                current_size = new_size;
            } // Swap buffers
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break, // End of file
            Err(e) => return Err(ScanError::CannotReadFile(e)),
        }
    }

    on_finish_term(state, mem, &mut real_out);

    Ok(real_out)
}
