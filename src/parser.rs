use crate::lexer::Token;

#[derive(Debug)]
pub enum Ast {
    Number { whole: String, fraction: String },
    Symbol(String),
    List(Vec<Ast>),
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Number { whole, fraction } => match fraction.len() {
                0 => write!(f, "{}", whole)?,
                _ => write!(f, "{}.{}", whole, fraction)?,
            },
            Ast::Symbol(name) => write!(f, "{name}")?,
            Ast::List(asts) => {
                write!(f, "(")?;
                for ast in asts {
                    write!(f, "{}", ast)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Parser {
    pub statements: Vec<Ast>,
    stack: Vec<Vec<Ast>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn finish(self) -> Ast {
        Ast::List(self.statements)
    }

    pub fn stack(&self) -> &Vec<Vec<Ast>> {
        &self.stack
    }

    pub fn parse(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::LP => self.stack.push(Vec::new()),
            Token::RP => {
                let Some(list) = self.stack.pop() else {
                    return Err("no matching `(` found for `)`".to_owned());
                };
                match self.stack.last_mut() {
                    Some(parent_list) => parent_list.push(Ast::List(list)),
                    None => self.statements.push(Ast::List(list)),
                }
            }
            Token::Number { whole, fraction } => match self.stack.last_mut() {
                Some(list) => list.push(Ast::Number { whole, fraction }),
                None => self.statements.push(Ast::Number { whole, fraction }),
            },
            Token::Symbol(name) => match self.stack.last_mut() {
                Some(list) => list.push(Ast::Symbol(name)),
                None => self.statements.push(Ast::Symbol(name)),
            },
        }
        Ok(())
    }
}
