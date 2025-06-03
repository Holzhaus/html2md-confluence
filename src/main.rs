use html2md_confluence::{ParseOptions, parse_confluence};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut buffer)?;

    let options = ParseOptions::default();
    print!("{}", parse_confluence(&buffer, &options));

    Ok(())
}
