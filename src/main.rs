mod code_gen;
mod parser;
mod scanner;
mod terminal;
mod token;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::scanner::ScanError;

const BUF_SIZE: usize = 1024;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

async fn process_buffers_and_scan(
    path: PathBuf,
    state: &mut scanner::ScanState,
    mem: &mut Vec<char>,
) -> Result<Vec<token::Token>, ScanError> {
    let mut file = File::open(path).await?;

    let mut buf_a = vec![0u8; BUF_SIZE];
    let mut buf_b = vec![0u8; BUF_SIZE];

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

        let out = scanner::scan(active_buf, current_size, state, mem)?;
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

    scanner::on_finish_term(state, mem, &mut real_out);

    Ok(real_out)
}

#[derive(Error, Debug)]
enum CompileError {
    #[error("Error during scanning: {0}")]
    Scan(scanner::ScanError),
    #[error("Error during parsing: {0}")]
    Parse(parser::ParseError),
    #[error("Error during code generation: {0}")]
    CodeGen(code_gen::CodeGenError),
}

impl From<scanner::ScanError> for CompileError {
    fn from(err: scanner::ScanError) -> Self {
        CompileError::Scan(err)
    }
}

impl From<parser::ParseError> for CompileError {
    fn from(err: parser::ParseError) -> Self {
        CompileError::Parse(err)
    }
}

impl From<code_gen::CodeGenError> for CompileError {
    fn from(err: code_gen::CodeGenError) -> Self {
        CompileError::CodeGen(err)
    }
}

async fn compile(input_path: String) -> Result<String, CompileError> {
    let mut state = scanner::ScanState::Start;
    let mut mem = vec![];
    let out = process_buffers_and_scan(PathBuf::from(input_path), &mut state, &mut mem).await?;
    let statements = parser::parse(out)?;
    // println!("{:#?}", statements);
    let code = code_gen::generate(statements)?;

    return Ok(code);
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let args = Args::parse();
    let out_code = compile(args.input).await;
    if let Some(err) = out_code.as_ref().err() {
        println!("{}", err);
        exit(1);
    }
    let out_code = out_code.unwrap();
    let duration = start.elapsed();
    if let Some(out) = args.output {
        fs::write(out, out_code).unwrap();
    } else {
        println!("{}", out_code);
    }
    println!("Finished, took {} microseconds.", duration.as_millis())
}
