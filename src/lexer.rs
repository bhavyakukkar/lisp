#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// Left Parenthesis i.e. `(`
    LP,
    /// Right Parenthesis i.e. `)`
    RP,
    /// Quote i.e. `'`
    Q,
    /// Backquote i.e. `\``
    BQ,
    /// Comma i.e. `,`
    C,
    /// Number i.e. `123` or `456.789`
    ///
    /// Negative numbers need to be represented using expressions i.e. `(- 3)`
    Number { whole: String, fraction: String },
    /// Symbols i.e. `a` or `A` or `_foo`
    Symbol(String),
}

/// Lexer that simply parses the next token
#[derive(Debug)]
pub struct Lexer<'a> {
    /// A reference to the source-code yet to be lexed
    pub source: &'a str,
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        enum LongToken {
            Integer(String),
            Float(String, String),
            Symbol(String),
        }
        use LongToken::*;

        let mut current_token = None;

        loop {
            // Get the next UTF-8 character in the source
            let ch = self.source.chars().next()?;
            let previous_source = self.source;
            self.source = &self.source[1..];

            current_token = match (ch, current_token) {
                // Fresh character
                ('(', None) => return Some(Ok(Token::LP)),
                (')', None) => return Some(Ok(Token::RP)),
                ('\'', None) => return Some(Ok(Token::Q)),
                ('`', None) => return Some(Ok(Token::BQ)),
                (',', None) => return Some(Ok(Token::C)),
                (digit @ '0'..='9', None) => Some(Integer(digit.into())),
                ('.', None) => Some(Float(String::new(), String::new())),
                (' ' | '\t' | '\n' | '\r', None) => None,
                (c @ _, None) => Some(Symbol(c.into())),

                // Character while reading an integer
                (digit @ '0'..='9', Some(Integer(mut digits))) => {
                    digits.push(digit);
                    Some(Integer(digits))
                }
                ('.', Some(Integer(digits))) => Some(Float(digits, String::new())),
                (_, Some(Integer(digits))) => {
                    self.source = previous_source;
                    return Some(Ok(Token::Number {
                        whole: digits,
                        fraction: String::new(),
                    }));
                }

                // Character while reading a float
                (digit @ '0'..='9', Some(Float(whole, mut fraction))) => {
                    fraction.push(digit);
                    Some(Float(whole, fraction))
                }
                ('.', Some(Float(_, _))) => {
                    return Some(Err("dot after floating number".to_string()));
                }
                (_, Some(Float(whole, fraction))) => {
                    self.source = previous_source;
                    return Some(Ok(Token::Number { whole, fraction }));
                }

                // Character while reading a symbol
                | ('(' | ')' | '\'' | '`' | ',', Some(Symbol(name))) // Special characters
                | (' ' | '\t' | '\n' | '\r', Some(Symbol(name))) => { // Ignored characters
                    self.source = previous_source;
                    return Some(Ok(Token::Symbol(name)));
                }
                (c @ _, Some(Symbol(mut name))) => {
                    name.push(c);
                    Some(Symbol(name))
                }
            }
        }
    }
}
