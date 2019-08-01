use super::expressions::*;
use super::statements::*;

// type ExprBox<'a> = Box<Expr<'a>>;
type StmtBox<'a> = Box<Statement<'a>>;


pub fn print<'a>(ast: &Vec<StmtBox<'a>>,) -> String {
    let mut output = String::new();

    for this_statement in ast {
        
    }

    output
}