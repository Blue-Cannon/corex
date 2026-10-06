
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("print")]
    Print,
    #[token("Hello!")]
    Hello,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLiteral(String),

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(";")] Semi,
    #[token(",")] Comma,
    #[token("=")] Equals,
    #[token("......")]
    DotDot,

    #[regex(r"[ \t\n\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)]
    Error,
}

pub fn lex_xc(source: &str) -> Vec<Token> {
    Token::lexer(source).filter_map(|t| t.ok()).collect()
  }
