mod ast;
mod subst;
mod sld;
mod input;

use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::fs::File;
use std::process::exit;
use sld::*;
use input::Input;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammer); // synthesized by LALRPOP

#[test]
fn grammer() {
    assert!(grammer::TermParser::new().parse("(x)").is_ok());
    assert!(grammer::TermParser::new().parse("f(x, y, Z)").is_ok());

    assert!(grammer::TermParser::new().parse("22").is_err());
    assert!(grammer::TermParser::new().parse("(22)").is_err());
    assert!(grammer::TermParser::new().parse("((((22))))").is_err());
    assert!(grammer::TermParser::new().parse("((22)").is_err());
}

fn load_file(filename : &str, records : &mut Records) {
    let f = File::open(filename).expect("error: file not found");
    let reader = io::BufReader::new(f);

    let mut add = Records::new();

    let parser = grammer::RuleParser::new();
    for line in reader.lines() {
        let line = line.expect("error: failed to read line");
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }

        match parser.parse(line) {
            Ok(rule) => {
                add.entry(rule.conclusion.prop_name.clone()).or_insert(Vec::new()).push(rule);
            },
            Err(e) => {
                println!("parse error: {}", e);
                return;
            }
        }
    }

    records.extend(add);
}

fn main() {
    let input_parser = grammer::InputParser::new();

    let mut records : Records = HashMap::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("IO error: failed to read stdin");

        match input_parser.parse(&buf) {
            Ok(input) => {
                match input {
                    Input::Load(filename) => {
                        println!("{:?}", filename);
                        load_file(&filename, &mut records);
                    },
                    Input::Inquire(goals) => {
                        sld::inquire(&mut |instance| {
                            
                            println!("{:?}", instance);

                            return true;
                        }, &records, goals)
                    }
                }
            },
            Err(e) => {
                println!("parse error : {}", e);
            }
        }
    }
}
