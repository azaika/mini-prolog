#[derive(Debug, Clone)]
pub enum Term {
    True,
    Atom(String),
    Variable(String),
    Compound(String, Vec<Term>)
}

impl Term {
    pub fn has_var(&self) -> bool {
        match self {
            Self::True => false,
            Self::Atom(_) => false,
            Self::Variable(_) => true,
            Self::Compound(_, ts) => ts.iter().any(Self::has_var)
        }
    }
}

pub enum Decl {
    Statement((String, Vec<Term>), Vec<Term>)
}

