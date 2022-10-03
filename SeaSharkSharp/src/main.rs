use crate::Tokens::*;
mod Tokens;

use crate::Lexer::*;
mod Lexer;

use crate::ArithmaticParser::*;
mod ArithmaticParser;

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
                    println!("float: {z}");
                }
                NUMBER::INT(z) =>
                {
                    println!("int: {z}");
                }
            }
        }
        Token::OPERATOR(y) =>
        {
            println!("OP");
        }
    }
}

fn main() 
{
    let TEXT = String::from("2 * 3 + -1 * 10");
    let tokens:Vec<Token> = lex_text(TEXT);
    for x in tokens.clone()
    {
        print_token(x);
    }
    let result = arithmatic_parser_multiply_divide(tokens);
    print_token(result);
}