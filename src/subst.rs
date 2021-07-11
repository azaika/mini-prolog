use std::collections::HashMap;
use crate::ast::{Term};

#[derive(Debug, Clone)]
pub struct Subst(HashMap<String, Term>);

impl Subst {
    pub fn apply_term(&self, t : Term) -> Term {
        let s = &self.0;
        match t {
            Term::Variable(v) => s.get(&v).cloned().unwrap_or(Term::Variable(v)),
            Term::Compound(p, mut args) => Term::Compound(p, args.drain(..).map(|t| { self.apply_term(t) }).collect()),
            _ => t
        }
    }

    pub fn compose(mut self, mut latter : Subst) -> Subst {
        let l_s = latter.0.drain().map(|(v, t)| { (v, self.apply_term(t)) }).collect::<HashMap<_, _>>();
        self.0.extend(l_s);
        self
    }
}