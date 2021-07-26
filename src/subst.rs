use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Subst(pub HashMap<String, Term>);

macro_rules! hashmap {
    ($( $pair: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($pair.0, $pair.1); )*
         map
    }}
}

impl Subst {
    pub fn apply_term(&self, t : Term) -> Term {
        let s = &self.0;
        match t {
            Term::Variable(v) => {
                if let Some(t_sub) = s.get(&v).cloned() {
                    self.apply_term(t_sub)
                }
                else {
                    Term::Variable(v)
                }
            },
            Term::Compound(c) => Term::Compound(self.apply_clause(c)),
            _ => t
        }
    }

    pub fn apply_clause(&self, c : Clause) -> Clause {
        let (p, mut args) = c.decompose();
        for t in &mut args {
            *t.as_mut() = self.apply_term(t.as_ref().clone());
        }
        
        Clause::from((p, args))
    }

    pub fn compose(mut self, mut latter : Subst) -> Subst {
        let l_s : HashMap<_, _> = latter.0.drain().map(|(v, t)| { (v, self.apply_term(t)) }).collect();
        self.0.extend(l_s);
        self
    }

    fn unify_term(t1 : &Term, t2 : &Term) -> Option<Subst> {
        use Term::*;
        match (t1, t2) {
            (Atom(s1), Atom(s2)) => if s1 == s2 { Some(Subst(HashMap::new())) } else { None }
            (Variable(s1), t2) => if t2.has_var(s1) { None } else { Some(Subst(hashmap![(s1.clone(), t2.clone())])) },
            (t1, Variable(s2)) => if t1.has_var(s2) { None } else { Some(Subst(hashmap![(s2.clone(), t1.clone())])) },
            (Compound(c1), Compound(c2)) => Self::unify(c1, c2),
            _ => None
        }
    }

    fn merge(&mut self, mut latter : Subst) -> bool {
        for (v, t1) in latter.0.drain() {
            if let Some(t2) = self.0.get(&v) {
                if let Some(u) = Self::unify_term(&t1, t2) {
                    if !self.merge(u) {
                        return false;
                    }
                }
                else {
                    return false;
                }
            }
            else {
                self.0.insert(v, t1);
            }
        }

        true
    }

    pub fn unify(c1 : &Clause, c2 : &Clause) -> Option<Subst> {
        let Clause{ prop_name : s1, args : v1} = c1;
        let Clause{ prop_name : s2, args : v2} = c2;

        if s1 != s2 { // 違う述語
            return None
        }
        
        let mut ret = Subst(HashMap::new());
        for (t1, t2) in v1.iter().zip(v2.iter()) {
            if let Some(subst) = Self::unify_term(t1, t2) {
                if !ret.merge(subst) {
                    return None;
                }
            }
            else {
                return None;
            }
        }

        Some(ret)
    }
}