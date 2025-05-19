// TODO: add strings maybe
use std::{
    fs::File,
    io::{BufReader, Read},
};

mod lexer;
pub use lexer::*;

mod parser;
pub use parser::*;

mod eval;
pub use eval::*;

fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let exe_name = args.next().expect("executable name not in args ??");
    let Some(file_name) = args.next() else {
        eprintln!("Usage: {} file.scm", exe_name);
        std::process::exit(1);
    };

    let mut parser = Parser::new();

    let mut reader = BufReader::new(File::open(file_name)?);
    let mut buf_extra = Vec::new();
    let mut buf = [0; 128];

    while reader.read(&mut buf)? > 0 {
        buf_extra.extend_from_slice(&buf);
        let source = String::from_utf8(buf_extra.clone()).unwrap_or_else(|e| {
            let valid_up_to = e.utf8_error().valid_up_to();
            let bytes = e.into_bytes();
            // Update buffer to hold the invalid bytes
            buf_extra = bytes[valid_up_to..].into();
            let (valid_bytes, _) = bytes.split_at(valid_up_to);
            unsafe { String::from_utf8_unchecked(valid_bytes.to_vec()) }
        });
        buf.iter_mut().for_each(|el| *el = 0);

        let source = &source;
        for token in (Lexer { source }) {
            match parser.parse(match token {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Lex error: {}", e);
                    std::process::exit(1);
                }
            }) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
    let stack = parser.stack();
    if stack.len() > 0 {
        eprintln!("Warning: unfinished trees in the stack: {:?}", stack);
    }
    println!("{}", parser.finish());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        use Token::*;
        let lexer = Lexer {
            source: "(* (+ 2 1.14) a &&& .88)",
        };
        assert_eq!(
            lexer.collect::<Vec<_>>(),
            vec![
                Ok(LP),
                Ok(Symbol("*".into())),
                Ok(LP),
                Ok(Symbol("+".into())),
                Ok(Number {
                    whole: "2".into(),
                    fraction: "".into(),
                }),
                Ok(Number {
                    whole: "1".into(),
                    fraction: "14".into(),
                }),
                Ok(RP),
                Ok(Symbol("a".into())),
                Ok(Symbol("&&&".into())),
                Ok(Number {
                    whole: "".into(),
                    fraction: "88".into(),
                }),
                Ok(RP),
            ]
        );
    }
}
