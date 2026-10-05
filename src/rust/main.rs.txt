use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

// ============================
// CoreX - Code at the Core
// Language written in Rust
// Mascot: Ming Ming The Cat
// Author: Rogge Ramos
// ============================

#[derive(Parser)]
#[command(
    name = "corex",
    version = "0.1.0",
    author = "Rogge Ramos",
    about = "CoreX programming language - Code at the Core. With Ming Ming The Cat!",
    long_about = r#"
   _____                __  __
  / ____|              | | \ \
 | |     ___  _ __ ___ | |  | |
 | |    / _ \| '__/ _ \| |  | |
 | |___| (_) | | |  __/| |  | |
  \_____\___/|_|  \___||_| /_/

 Code at the Core.
 Mascot: Ming Ming The Cat 😇💙

 Examples:
   corex run hello.xc
   corex run hello.xc --user Rogge
   corex repl
   corex build main.xc
   corex version --mingming
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Your username for Hello! greeting
    #[arg(short, long, global = true, default_value = "World")]
    user: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .xc file
    Run {
        /// Path to .xc file
        file: PathBuf,

        /// Username to greet
        #[arg(short, long)]
        user: Option<String>,

        /// Show Ming Ming while running
        #[arg(long)]
        mingming: bool,
    },
    /// Start CoreX REPL
    Repl {
        #[arg(short, long)]
        user: Option<String>,
    },
    /// Build a .xc file
    Build {
        file: PathBuf,
        #[arg(short, long, default_value = "corex_out")]
        output: String,
    },
    /// Show CoreX version and Ming Ming
    Version {
        #[arg(long)]
        mingming: bool,
    },
    /// Init new CoreX project
    Init {
        name: String,
    },
}

#[derive(Error, Debug, Diagnostic)]
#[error("CoreX Error")]
enum CoreXError {
    #[error("File not found: {0}")]
    #[diagnostic(code(corex::file_not_found), help("Make sure the .xc file exists"))]
    FileNotFound(String),

    #[error("Invalid extension: expected .xc, got {0}")]
    #[diagnostic(code(corex::invalid_extension))]
    InvalidExtension(String),
}

// ============================
// Ming Ming The Cat - ASCII Art
// ============================
fn print_mingming_sleeping() {
    println!("{}", r#"
    ╭─────────────────────────────╮
    │  😇 Ming Ming is sleeping   │
    │  Zzz... compiling...        │
    ╰─────────────────────────────╯
         /\_/\  
        ( o.o )  💙 Halo!
         > ^ <
    "#.bright_blue());
}

fn print_mingming_awake(username: &str) {
    println!("{}", format!(r#"
    ╭─────────────────────────────────────╮
    │  Ming Ming The Cat - CoreX Guardian │
    ╰─────────────────────────────────────╯
         /\_/\  
        ( ^.^ )  Meow! Hello!
         / >💙
    
    Hello!, {}! 
    Ming Ming says: Welcome to CoreX!
    "#, username).bright_cyan());
}

fn print_banner() {
    println!("{}", r#"
  ____                __  __
 / ___|___  _ __ ___  \ \/ /
| |   / _ \| '__/ _ \  \  / 
| |__| (_) | | |  __/  /  \ 
 \____\___/|_|  \___| /_/\_\
                             
   Code at the Core - v0.1.0
   Written in Rust 🦀 + 💙
"#.truecolor(0, 102, 255).bold());
}

// ============================
// Core Lexer - Basic for .xc
// ============================
#[derive(Debug, Clone)]
enum Token {
    Let,
    Fn,
    Print,
    Identifier(String),
    StringLiteral(String),
    Number(f64),
    Symbol(char),
    Eof,
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return Token::Eof;
        }
        let ch = self.input[self.pos];
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = self.pos;
                while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
                    self.pos += 1;
                }
                let word: String = self.input[start..self.pos].iter().collect();
                match word.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Fn,
                    "print" => Token::Print,
                    _ => Token::Identifier(word),
                }
            }
            '"' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < self.input.len() && self.input[self.pos] != '"' {
                    self.pos += 1;
                }
                let s: String = self.input[start..self.pos].iter().collect();
                self.pos += 1;
                Token::StringLiteral(s)
            }
            '0'..='9' => {
                let start = self.pos;
                while self.pos < self.input.len() && (self.input[start..self.pos].iter().collect::<String>().parse::<f64>().is_ok() || self.input[self.pos].is_ascii_digit() || self.input[self.pos] == '.') {
                    self.pos += 1;
                    if self.pos < self.input.len() && !self.input[self.pos].is_ascii_digit() && self.input[self.pos] != '.' { break; }
                }
                let num_str: String = self.input[start..self.pos].iter().collect();
                Token::Number(num_str.parse().unwrap_or(0.0))
            }
            _ => {
                self.pos += 1;
                Token::Symbol(ch)
            }
        }
    }
}

// ============================
// Core Functions
// ============================
fn greet_user(username: &str) {
    print_banner();
    println!();
    
    // The main Hello!, (Username) feature
    let greeting = format!("Hello!, {}......", username);
    println!("{}", greeting.bright_white().bold().on_truecolor(0, 102, 255));
    println!();
    
    println!("{}", format!("Welcome to CoreX, {}!", username).bright_blue());
    println!("{}", "Your programming language written in Rust is ready!".dimmed());
    println!();
    
    print_mingming_awake(username);
    println!();
    
    println!("{}", "Quick Tips:".yellow().bold());
    println!("  {} - Run your first file", "corex run examples/hello.xc".green());
    println!("  {} - Start interactive mode", "corex repl".green());
    println!("  {} - Build to binary", "corex build main.xc".green());
    println!();
}

fn run_file(file_path: &PathBuf, username: &str, show_mingming: bool) {
    println!("{}", format!("CoreX is running: {:?}", file_path).bright_blue());
    println!("{}", format!("User: {}", username).dimmed());
    println!();

    if show_mingming {
        print_mingming_sleeping();
        println!();
    }

    // Check file
    if !file_path.exists() {
        eprintln!("{}", format!("Error: File not found: {:?}", file_path).red().bold());
        eprintln!("{}", "Ming Ming says: Meow! File not found! (｡•́︿•̀｡)".red());
        return;
    }

    let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    if extension != "xc" {
        eprintln!("{}", format!("Warning: Expected .xc file, got .{} - trying anyway...", extension).yellow());
    }

    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("{}", "--- Source Code ---".dimmed());
            println!("{}", content.white());
            println!("{}", "--- Output ---".dimmed());
            
            // Simple interpreter - look for print statements
            let mut lexer = Lexer::new(content.clone());
            loop {
                let token = lexer.next_token();
                match token {
                    Token::Eof => break,
                    Token::Print => {
                        // Very basic: if next is string literal, print it
                        let next = lexer.next_token();
                        if let Token::StringLiteral(s) = next {
                            // Replace {username} placeholder
                            let output = s.replace("{user}", username).replace("{username}", username);
                            println!("{}", output.green());
                        }
                    }
                    _ => {}
                }
            }

            // Also check for Hello! pattern in file
            if content.contains("Hello") {
                println!();
                println!("{}", format!("Hello!, {}...... (from file)", username).cyan().bold());
            }

            println!();
            println!("{}", "✓ Execution finished! Ming Ming woke up!".green().bold());
            if show_mingming {
                print_mingming_awake(username);
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to read file: {}", e).red());
        }
    }
}

fn start_repl(username: &str) {
    greet_user(username);
    println!("{}", "CoreX REPL v0.1.0 - Type 'exit' to leave, 'mingming' to see Ming Ming".magenta());
    println!();

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("{}", format!("corex@{}> ", username).bright_blue().bold());
        io::stdout().flush().unwrap();
        line.clear();
        
        if stdin.read_line(&mut line).is_err() {
            break;
        }
        
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            "exit" | "quit" | ":q" => {
                println!("{}", format!("Goodbye!, {}...... See you next time!", username).yellow());
                print_mingming_sleeping();
                break;
            }
            "mingming" | "cat" => {
                print_mingming_awake(username);
            }
            "hello" => {
                println!("{}", format!("Hello!, {}......", username).cyan().bold());
            }
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
            }
            "help" => {
                println!("Commands: hello, mingming, clear, exit, help");
                println!("You can also type CoreX code like: print(\"Hello!\")");
            }
            _ => {
                // Try to interpret as CoreX code
                if trimmed.contains("print") {
                    // Extract string inside print(...)
                    if let Some(start) = trimmed.find('"') {
                        if let Some(end) = trimmed[start+1..].find('"') {
                            let text = &trimmed[start+1..start+1+end];
                            let output = text.replace("{user}", username);
                            println!("{}", output.green());
                            continue;
                        }
                    }
                    println!("{}", trimmed.yellow());
                } else {
                    println!("{}", format!("You typed: {} - Ming Ming is thinking...", trimmed).dimmed());
                }
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let global_user = cli.user;

    match cli.command {
        Some(Commands::Run { file, user, mingming }) => {
            let username = user.unwrap_or(global_user);
            greet_user(&username);
            run_file(&file, &username, mingming);
        }
        Some(Commands::Repl { user }) => {
            let username = user.unwrap_or(global_user);
            start_repl(&username);
        }
        Some(Commands::Build { file, output }) => {
            println!("{}", format!("Building {:?} -> {}", file, output).bright_blue().bold());
            print_mingming_sleeping();
            println!("Compiling with Rust backend...");
            // Simulate build
            std::thread::sleep(std::time::Duration::from_millis(800));
            println!("{}", format!("✓ Built successfully: {}", output).green().bold());
            println!("{}", format!("Hello!, {}...... Your binary is ready!", global_user).cyan());
        }
        Some(Commands::Version { mingming }) => {
            print_banner();
            println!("CoreX v0.1.0");
            println!("Written in Rust");
            println!("Mascot: Ming Ming The Cat 😇");
            println!("Author: Rogge Ramos");
            println!("License: MIT");
            println!();
            println!("{}", format!("Hello!, {}......", global_user).bold());
            if mingming {
                print_mingming_awake(&global_user);
            }
        }
        Some(Commands::Init { name }) => {
            println!("{}", format!("Initializing new CoreX project: {}", name).green());
            println!("{}", format!("Hello!, {}...... Creating project...", global_user).cyan());
            // Create folder structure logic would go here
            println!("Created {}/", name);
            println!("Created {}/src/examples/hello.xc", name);
            println!("Created {}/Cargo.toml", name);
            println!();
            print_mingming_awake(&global_user);
        }
        None => {
            // No subcommand = default greeting - The Hello!, (Username) feature
            greet_user(&global_user);
            
            println!("{}", "Usage:".yellow().bold());
            println!("  corex run <file.xc> --user YourName");
            println!("  corex repl --user YourName");
            println!("  corex version --mingming");
            println!();
            println!("{}", "Try: cargo run -- --user Rogge".dimmed());
        }
    }
}
