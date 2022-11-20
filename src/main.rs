use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::lex::{Lex, lex_analyzer, Token};
use crate::parser::parser;
use std::env;

mod lex;
mod parser;

fn main() -> std::io::Result<()>{

    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[2])?;
    let reader = BufReader::new(file);
    let mut symbol_table: Vec<Lex> = Vec::new();
    let mut n_line: usize = 1;

    for line in reader.lines() {
        lex_analyzer(line.unwrap(), &mut symbol_table, n_line);
        n_line += 1;
    }

    if args[1].to_string() == String::from("--lex"){

        let mut ouput = File::create("output.txt")?;
        for item in symbol_table.into_iter() {
            write!(ouput, "{}\t{}\t{}\n", item.lexeme, item.token.to_string(), item.pattern)?;
        }

    }
    else if args[1].to_string() == String::from("--parser"){

        let mut ouput = File::create("output.txt")?;
        write!(ouput,"{}",parser(symbol_table))?;
    }

    /*
     */


    Ok(())
}
