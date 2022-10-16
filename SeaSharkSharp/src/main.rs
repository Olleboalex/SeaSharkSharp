use crate::Tokens::*;
mod Tokens;

use crate::Lexer::*;
mod Lexer;

use crate::ArithmaticParser::*;
mod ArithmaticParser;

use crate::Parsing::*;
mod Parsing;

use crate::StandardLibrary::*;
mod StandardLibrary;

fn print_token(x:Token)
{
    match x
    {
        Token::NUM(y) =>
        {
            match y
            {
                NUMBER::FLOAT(z) =>
                {
                    println!("{z}");
                }
                NUMBER::INT(z) =>
                {
                    println!("{z}");
                }
            }
        }
        Token::OPERATOR(y) =>
        {
            println!("OP");
        }
        Token::BOOL(y) =>
        {
            println!("{y}");
        }
        Token::IF(y, z) =>
        {
            println!("if");
            print_tokens(y);
            println!("then");
            print_tokens(z);
            println!("END");
        }
        _ => ()
    }
}

fn print_tokens(tokens:Vec<Token>)
{
    for element in tokens
    {
        print_token(element);
    }
}

fn main() 
{
    let TEXT = String::from("while (true) {1 + 1}");
    let tokens:Vec<Token> = lex_text(TEXT);
    let result = Parse(tokens.clone());
}