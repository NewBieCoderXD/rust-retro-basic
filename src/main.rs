mod parser;
mod scanner;
mod token;
use std::io::SeekFrom;
use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, BufReader};

use clap::Parser;

// fn compile(){

// }

const BUF_SIZE: usize = 1024;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

async fn process_buffers_and_scan(
    path: PathBuf,
    state: &mut scanner::ScanState,
    mem: &mut Vec<char>,
    out: &mut Vec<token::Token>,
) -> tokio::io::Result<()> {
    let mut file = File::open(path).await?;

    let mut buf_a = vec![0u8; BUF_SIZE];
    let mut buf_b = vec![0u8; BUF_SIZE];

    let n = file.read(&mut buf_a).await?;
    if n == 0 {
        return Ok(());
    }

    let mut current_is_a = true;

    loop {
        let active_buf;
        let next_read;
        if current_is_a {
            next_read = file.read_exact(&mut buf_b);
            active_buf = &buf_a;
        } else {
            next_read = file.read_exact(&mut buf_a);
            active_buf = &buf_b;
        };

        scanner::scan(active_buf, state, mem, out);

        match next_read.await {
            Ok(_) => current_is_a = !current_is_a, // Swap buffers
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break, // End of file
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut state = scanner::ScanState::Start;
    let mut mem = vec![];
    let mut out = vec![];
    process_buffers_and_scan(PathBuf::from(args.input), &mut state, &mut mem, &mut out)
        .await
        .unwrap();

    println!("{:?}", out);
}
