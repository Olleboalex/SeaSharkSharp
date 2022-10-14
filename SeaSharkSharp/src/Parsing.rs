use crate::Tokens::*;
use crate::ArithmaticParser::*;

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
            _ => ()
        }
        cursor += 1;
    }
    Token::BOOL(false)
}