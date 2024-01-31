use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;
use crate::TokenType::{LiteralInt, Return, Semicolon};

#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType {
    Return,
    LiteralInt,
    Semicolon,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,      // Optional
}
fn generate_asm(tokens: Vec<Token>) -> String {
    let mut output = String::new();
    output.push_str("global _start\n_start:\n");
    let mut iter_tokens = tokens.iter().peekable();
    while let Some(token) = iter_tokens.next() {
        if token.token_type == Return {
            output.push_str("    mov rax, 60\n");
            while let Some(token) = iter_tokens.next() {
                if token.token_type == LiteralInt {
                    let s = String::from("    mov rdi, ".to_owned() + &*token.value.clone().unwrap() + "\n");
                    output.push_str(&*s);
                    while let Some(token) = iter_tokens.next() {
                        if token.token_type == Semicolon {
                            output.push_str("    syscall\n");
                        }
                    }
                }
            }
        }
    }

    output
}
fn tokenize(str: String) -> Vec<Token> {
    let mut buf = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = str.chars().peekable();

    while let Some(&current_char) = iter.peek() {
        // Keywords
        // First character must be alphabetic and all subsequent characters can be alphanumeric
        if current_char.is_alphabetic() {
            while let Some(&next_char) = iter.peek() {
                if next_char.is_alphanumeric() {
                    buf.push(next_char);
                    iter.next();
                } else {
                    break;
                }
            }
            tokens.push(Token{
                token_type: Return,
                value: None,
            });
            buf.clear();
        }
        // Integer literals
        else if current_char.is_digit(10) {
            while let Some(&next_char) = iter.peek() {
                if next_char.is_digit(10) {
                    buf.push(next_char);
                    iter.next();
                } else {
                    break;
                }
            }
            tokens.push(Token{
                token_type: LiteralInt,
                value: Some(buf.clone()),
            });
            buf.clear();
        }
        // Semicolon
        else if current_char == ';' {
            tokens.push(Token{
                token_type: Semicolon,
                value: None,
            });
            iter.next();
        }
        // Whitespace
        else if current_char.is_whitespace() {
            iter.next();
        }
        // If nothing is encountered, then there must be an error
        else {
            eprintln!("Coffee spilled! Something's wrong.");
            exit(1);
        }
    }

    tokens
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.cb>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut tokens_array: Vec<Token>;
    for line in reader.lines() {
        match line {
            Ok(contents) => {
                tokens_array = tokenize(contents);
                let output = generate_asm(tokens_array);
                let mut output_file = File::create("out.asm").expect("Failed to create output file");
                output_file.write_all(output.as_bytes()).expect("Failed to write.");
            },
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
}
