use std::fmt;
use std::collections::{HashMap, VecDeque};

use crate::ast;
use crate::subst::Subst;

pub type Records = HashMap<String, Vec<ast::Rule>>;

#[derive(Debug)]
pub struct Instance(Vec<(String, String)>);

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return fmt::Result::Ok(());
        }

        let mut print = |p : &(String, String), is_tail : bool| {
            if is_tail {
                write!(f, ", {} = {}", p.0, p.1)
            }
            else {
                write!(f, "{} = {}", p.0, p.1)
            }
        };

        self.0[1..].iter().fold(print(&self.0[0], false), |r, p| {
            r.and_then(|_| { print(p, true) })
        })
    }
}

fn sld_derivative<F>(callback : &mut F, records : &Records, goals : &mut VecDeque<ast::Clause>, subst : Subst, count : usize) -> bool
where F : FnMut(Subst) -> bool {
    if let Some(goal) = goals.pop_front() {
        for rule in records.get(&goal.prop_name).unwrap_or(&Vec::new()) {
            let (rule, new_count) = rule.clone().renew(count);

            let sigma = Subst::unify(&goal, &rule.conclusion);
            if sigma.is_none() {
                continue;
            }
            let sigma = sigma.unwrap();

            let mut new_goals = goals.clone();
            for t in rule.conds.iter().rev() {
                new_goals.push_front(t.clone());
            };

            new_goals = new_goals.drain(..).map(|c| { sigma.apply_clause(c) }).collect();

            let new_subst = subst.clone().compose(sigma);

            if !sld_derivative(callback, records, &mut new_goals, new_subst, new_count) {
                return false;
            }
        }

        return true;
    }
    else {
        return callback(subst);
    }
}

pub fn inquire<F>(callback : &mut F, records : &Records, goals : VecDeque<ast::Clause>)
where F : FnMut(Instance) -> bool {
    let mut callback_internal = |subst : Subst| -> bool {
        let mut vec : Vec<_> = subst.0.iter().filter_map(|(v, t)| {
            if !v.chars().nth(0).unwrap().is_digit(10) {
                Some((v.to_owned(), subst.apply_term(t.clone()).to_string()))
            }
            else {
                None
            }
        }).collect();
        vec.sort();

        callback(Instance(vec))
    };

    sld_derivative(&mut callback_internal, records, &mut goals.clone(), Subst(HashMap::new()), 0);
}