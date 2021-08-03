mod ast;
mod subst;
mod sld;
mod input;

use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};
use std::fs::File;
use sld::*;
use input::Input;

use clap::{ Arg, App };

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammer); // synthesized by LALRPOP

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
    let matches = App::new("mini-prolog")
        .arg(Arg::with_name("bfs").short("b").takes_value(false)).get_matches();

    let bfs = matches.is_present("bfs");
    
    let input_parser = grammer::InputParser::new();

    let mut records : Records = HashMap::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("IO error: failed to read stdin");

        if buf.trim().is_empty() {
            continue;
        }

        match input_parser.parse(&buf) {
            Ok(input) => {
                match input {
                    Input::Load(filename) => {
                        load_file(&filename, &mut records);
                    },
                    Input::Inquire(goals) => {
                        sld::inquire(&mut |instance| {
                            print!("{} ", instance);
                            io::stdout().flush().expect("output error");
                            loop {
                                let mut buf : [u8; 1] = [0];
                                io::stdin().read(&mut buf).expect("input error");

                                match buf[0] as char {
                                    ';' => { return true; }
                                    '.' => { return false; }
                                    _ => { continue; }
                                }
                            }
                        }, &records, goals, bfs);
                    }
                }
            },
            Err(e) => {
                println!("parse error : {}", e);
            }
        }
    }
}
