mod code_gen;
mod parser;
mod scanner;
mod terminal;
mod token;
mod constants;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
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
    let tokens = scanner::process_buffers_and_scan(PathBuf::from(input_path), &mut state, &mut mem).await?;
    let statements = parser::parse(tokens)?;
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
