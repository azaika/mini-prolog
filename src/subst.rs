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
        let l_s : HashMap<_, _> = latter.0.drain().map(|(v, t)| { (v, self.apply_term(t)) }).collect();
        self.0.extend(l_s);
        self
    }

    pub fn unify(t1 : Term, t2 : Term) -> Option<Subst> {
        match (t1, t2) {
            (Term::Variable(s1), t2) => Some(Subst([(s1, t2)].iter().cloned().collect())),
            (t1, Term::Variable(s2)) => Some(Subst([(s2, t1)].iter().cloned().collect())),
            (Term::Compound(s1, v1), Term::Compound(s2, v2)) => {
                if s1 != s2 { // 違う述語
                    None
                }
                else {
                    let mut ret = HashMap::new();
                    for (t1, t2) in v1.iter().zip(v2.iter()) {
                        if let Some(subst) = Self::unify(t1.clone(), t2.clone()) {
                            ret.extend(subst.0);
                        }
                        else {
                            return None;
                        }
                    }

                    Some(Subst(ret))
                }
            },
            _ => None
        }
    }
}