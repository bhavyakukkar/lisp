use crate::{eval, lexer::Token};

#[derive(Debug)]
pub enum Ast {
    /// Atomic number (integer or floating-point)
    Number { whole: String, fraction: String },
    /// Atomic symbol
    Symbol(String),
    /// List of sub-lists or atomics
    List(Vec<(Ast, QuoteState)>),
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
                write!(f, "( ")?;
                for (ast, eval) in asts {
                    match eval {
                        QuoteState::None => (),
                        QuoteState::Backquote => write!(f, "`")?,
                        QuoteState::Quote => write!(f, "'")?,
                    }
                    write!(f, "{} ", ast)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuoteState {
    None,
    Backquote,
    Quote,
}

impl std::ops::Deref for QuoteState {
    type Target = Option<QuoteState>;
    fn deref(&self) -> &Self::Target {
        match self {
            QuoteState::None => &None,
            QuoteState::Backquote => &Some(QuoteState::Backquote),
            QuoteState::Quote => &Some(QuoteState::Quote),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub program: Vec<(eval::Ast, bool)>,
    stack: Vec<(
        Vec<(Ast, /* whether this cons is quoted */ QuoteState)>,
        /* whether this list is backquoted */ QuoteState,
    )>,
    quote_state: QuoteState,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            // Program itself is an unquoted list
            program: Vec::new(),
            stack: vec![],
            quote_state: QuoteState::None,
        }
    }

    pub fn finish(self) -> Ast {
        // Ast::List(self.program)
        todo!()
    }

    pub fn stack(&self) -> &Vec<(Vec<(Ast, QuoteState)>, QuoteState)> {
        // &self.stack
        todo!()
    }

    pub fn parse(&mut self, token: Token) -> Result<(), String> {
        match (token, self.quote_state) {
            (Token::Q, QuoteState::None) => self.quote_state = QuoteState::Quote,
            (Token::Q, QuoteState::Quote) => return Err(format!("`'` right after `'`")),
            (Token::Q, QuoteState::Backquote) => return Err(format!("`'` right after `\\``")),

            (Token::BQ, QuoteState::None) => self.quote_state = QuoteState::Backquote,
            (Token::BQ, QuoteState::Quote) => return Err(format!("`\\`` right after `'`")),
            (Token::BQ, QuoteState::Backquote) => return Err(format!("`\\`` right after `\\``")),

            (Token::LP, q) => {
                self.stack.push((Vec::new(), q));
                self.quote_state = QuoteState::None;
            }

            (Token::RP, QuoteState::None) => {
                let Some((list, q)) = self.stack.pop() else {
                    return Err("no matching `(` found for `)`".to_string());
                };
                match self.stack.last_mut() {
                    Some((parent_list, _)) => parent_list.push((Ast::List(list), q)),
                    None => self.program.push((Ast::List(list), q)),
                }
            }
            (Token::RP, QuoteState::Quote) => return Err(format!("`)` right after `'`")),
            (Token::RP, QuoteState::Backquote) => return Err(format!("`)` right after `\\``")),

            // TODO HERE: and comma is only allowed in list with backquote, and it basically makes the cons have no quote
            // also logic to make every cons inside a backquoted list (single) quoted
            (Token::C, QuoteState::None) => match self.stack.last() {
                Some((_, QuoteState::Backquote)) => (),
                _ => return Err("`,` can only be used in a list quoted with `\\``".to_string()),
            },
            (Token::C, QuoteState::Quote) => return Err(format!("`,` right after `'`")),
            (Token::C, QuoteState::Backquote) => return Err(format!("`,` right after `\\``")),

            (Token::Number { whole, fraction }, q) => {
                match self.stack.last_mut() {
                    Some((list, _)) => list.push((Ast::Number { whole, fraction }, q)),
                    None => self.program.push((Ast::Number { whole, fraction }, q)),
                }
                self.quote_state = QuoteState::None;
            }

            (Token::Symbol(name), q) => {
                match self.stack.last_mut() {
                    Some((list, _)) => list.push((Ast::Symbol(name), q)),
                    None => self.program.push((Ast::Symbol(name), q)),
                }
                self.quote_state = QuoteState::None;
            }
        };
        Ok(())
    }
}
