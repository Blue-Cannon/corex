
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum AST {
    HelloStmt { username: String },
    PrintStmt { text: String },
    LetStmt { name: String, value: String },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }

    pub fn parse(&mut self) -> Vec<AST> {
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Hello => {
                    // Hello!, (Username)...... pattern
                    self.pos += 1;
                    let mut username = "World".to_string();
                    if let Some(Token::Identifier(u)) = self.tokens.get(self.pos).cloned() {
                        username = u; self.pos += 1;
                    }
                    nodes.push(AST::HelloStmt { username });
                }
                Token::Print => {
                    self.pos += 1;
                    if let Some(Token::StringLiteral(s)) = self.tokens.get(self.pos).cloned() {
                        self.pos += 1;
                        nodes.push(AST::PrintStmt { text: s });
                    }
                }
                _ => { self.pos += 1; }
            }
        }
        nodes
    }
                                                   }
                  
