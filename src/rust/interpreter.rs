
use crate::parser::AST;
use colored::*;

pub struct Interpreter {
    pub username: String,
}

impl Interpreter {
    pub fn new(username: String) -> Self { Self { username } }

    pub fn run(&self, nodes: Vec<AST>) {
        for node in nodes {
            match node {
                AST::HelloStmt { username } => {
                    println!("{}", format!("Hello!, {}......", username).bright_cyan().bold());
                    println!("{}", format!("Ming Ming says: Hi {}! Meow!", username).bright_blue());
                }
                AST::PrintStmt { text } => {
                    let out = text.replace("{user}", &self.username).replace("{username}", &self.username);
                    println!("{}", out.green());
                }
                AST::LetStmt { name, value } => {
                    println!("{}", format!("let {} = {} - Ming Ming stored it!", name, value).dimmed());
                }
            }
        }
    }
              }
              
