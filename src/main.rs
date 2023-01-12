use anyhow::{anyhow, Result};
use std::env;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = args.first().ok_or_else(|| anyhow!("X"))?;
    println!("input: {input}");
    Ok(())
}

fn main() -> Result<()> {
    run()
}
