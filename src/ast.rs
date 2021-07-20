use std::fmt;
use std::collections::{VecDeque, HashSet, HashMap};

#[derive(Debug, Clone)]
pub struct Clause {
    pub prop_name : String,
    pub args : Vec<Box<Term>>
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.prop_name)
        .and_then(|_| {
            write!(f, "{}", self.args.first().map(|t| t.as_ref()).unwrap_or(&Term::Atom("".to_owned())))
        })
        .and_then(|_| {
            let mut r = Ok(());
            for t in self.args[1..].iter() {
                if r.is_ok() {
                    r = write!(f, ",{}", *t);
                }
                else {
                    break;
                }
            }
            r
        })
        .and_then(|_| write!(f, ")"))
    }
}

impl Clause {
    pub fn decompose(self) -> (String, Vec<Box<Term>>) {
        (self.prop_name, self.args)
    }

    fn search_vars(&self, vars : &mut HashSet<String>) {
        for t in &self.args {
            t.as_ref().search_vars(vars);
        }
    }

    fn rewrite(mut self, table : &HashMap<String, String>) -> Self {
        Self {
            prop_name : self.prop_name,
            args : self.args.drain(..).map(|t| { Box::new((*t).rewrite(table)) }).collect()
        }
    }

    pub fn renew(self, count : usize) -> (Self, usize) {
        let mut vars = HashSet::new();
        self.search_vars(&mut vars);

        let num = vars.len();
        let table : HashMap<String, String> = vars.drain().enumerate().map(|(i, v)| { (v, (count + i).to_string()) }).collect();

        (self.rewrite(&table), count + num)
    }
}

impl From<(String, Vec<Box<Term>>)> for Clause {
    fn from(p : (String, Vec<Box<Term>>)) -> Self {
        Self {
            prop_name : p.0,
            args : p.1
        }
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    Atom(String),
    Variable(String),
    Compound(Clause)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Variable(v) => write!(f, "{}", v.chars().nth(0).filter(|c| c.is_ascii_digit()).map_or("", |_| "_").to_owned() + v),
            Self::Compound(c) => write!(f, "{}", c)
        }
    }
}

impl Term {
    pub fn has_var(&self, var : &String) -> bool {
        match self {
            Self::Atom(_) => false,
            Self::Variable(v) => v == var,
            Self::Compound(Clause{ prop_name : _, args }) => args.iter().map(Box::as_ref).any(|v| { v.has_var(var) })
        }
    }

    fn search_vars(&self, vars : &mut HashSet<String>) {
        match self {
            Self::Variable(v) => { vars.insert(v.clone()); },
            Self::Compound(c) => { c.search_vars(vars); },
            _ => {}
        }
    }

    fn rewrite(self, table : &HashMap<String, String>) -> Self {
        match self {
            Self::Variable(v) => Self::Variable(table.get(&v).unwrap().clone()),
            Self::Compound(c) => Self::Compound(c.rewrite(table)),
            _ => self
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub conclusion : Clause,
    pub conds : VecDeque<Clause>
}

impl Rule {
    pub fn renew(mut self, count : usize) -> (Self, usize) {
        let mut vars = HashSet::new();
        self.conclusion.search_vars(&mut vars);
        for c in &self.conds {
            c.search_vars(&mut vars);
        }

        let num = vars.len();
        let table : HashMap<String, String> = vars.drain().enumerate().map(|(i, v)| { (v, (count + i).to_string()) }).collect();

        let ret  = Self {
            conclusion : self.conclusion.rewrite(&table),
            conds : self.conds.drain(..).map(|c| { c.rewrite(&table) }).collect()
        };

        (ret, count + num)
    }
}