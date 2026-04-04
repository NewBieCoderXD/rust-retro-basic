use crate::token;

#[derive(Debug)]
pub enum ScanState {
    Start,
    StartIdentifier,
    StartNumber,
}

pub fn scan(
    raw_chars: &Vec<u8>,
    state: &mut ScanState,
    mem: &mut Vec<char>,
    out: &mut Vec<token::Token>,
) {
    let on_finish_term =
        |state: &mut ScanState, mem: &mut Vec<char>, out: &mut Vec<token::Token>| match state {
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
    let chars: Vec<char> = raw_chars.iter().map(|&num| num as char).collect();
    for char in chars {
        if char == '\r' {
            continue;
        }
        if char == ' ' || char == '\n' {
            on_finish_term(state, mem, out);
        }
        match state {
            ScanState::Start => {
                if char.is_ascii_digit() {
                    *state = ScanState::StartNumber;
                    mem.push(char);
                } else if char.is_ascii_alphabetic() {
                    *state = ScanState::StartIdentifier;
                    mem.push(char);
                } else if char == '=' || char == '>' || char == '<' {
                    let compare = if char == '=' {
                        token::TokenCompare::Equal
                    } else if char == '>' {
                        token::TokenCompare::MoreThan
                    } else {
                        token::TokenCompare::LessThan
                    };
                    out.push(token::Token::Compare(compare));
                    *state = ScanState::Start;
                    mem.clear();
                }
            }
            ScanState::StartIdentifier => {
                if char.is_ascii_alphanumeric() {
                    mem.push(char);
                }
            }
            ScanState::StartNumber => {
                if char.is_ascii_digit() {
                    mem.push(char);
                }
            }
        }
    }
    on_finish_term(state, mem, out);
}
