use std::collections::VecDeque;
use crate::ast::Clause;

#[derive(Debug, Clone)]
pub enum Input {
    Load(String),
    Inquire(VecDeque<Clause>)
}