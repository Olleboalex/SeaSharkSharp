use crate::Tokens::*;
use crate::ArithmaticParser::*;
use crate::print_token;
use crate::StandardLibrary::*;

fn parse_method_call(methodcall:Token) -> Token
{
    Token::BOOL(true)
}

pub fn Parse(tokens:Vec<Token>) -> Token
{
    let mut cursor = 0;
    while cursor < tokens.len()
    {
        match &tokens[cursor]
        {
            Token::IF(condition, statement) =>
            {
                if let Token::BOOL(x) = condition[0]
                {
                    if x
                    {
                        return parse_arithmatic((*statement).clone());
                    }
                }
            }
            Token::WHILE(condition, statement) =>
            {
                if let Token::BOOL(x) = condition[0]
                {
                    //let mut i = 0;
                    while x
                    {
                        print_token(parse_arithmatic((*statement).clone()));
                        //println!("{i}");
                        //i += 1;
                    }
                }
            }
            _ => ()
        }
        cursor += 1;
    }
    Token::BOOL(false)
}