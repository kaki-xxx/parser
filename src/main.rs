mod parser;

#[cfg(test)]
mod test;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 || args.len() >= 2 {
        eprintln!("Usage:");
        eprintln!("cargo run '1+2'");
        return;
    }
    let mut p = parser::Parser::new(args[0].clone());
    if let Some(r) = p.parse() {
        println!("{}", r);
    } else {
        println!("Syntax Error.");
    }
}
