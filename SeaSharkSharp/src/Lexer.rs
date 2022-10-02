use crate::Tokens::*;

fn KeyWordFinder(text:&[u8], pos:usize, find:String) -> bool
{
    false
}

fn IsNumber(x:u8) -> bool
{
    x >= '0' as u8 && x <= '9' as u8
}

pub fn LexText(input:String) -> Vec<Token>
{
    let text:&[u8] = input.as_bytes();
    let mut tokens:Vec<Token> = vec![];
    let mut cursor = 0;
    while cursor < text.len()
    {
        if IsNumber(text[cursor])
        {
            let mut temp:Vec<u8> = vec![text[cursor]];
            while cursor < text.len()
            {
                if !IsNumber(text[cursor])
                {
                    cursor -= 1;
                    break;
                }
                temp.push(text[cursor]);
                cursor += 1;
            }
            tokens.push(Token::ParseNumber(temp));
        }
        else if text[cursor] == '+' as u8
        {
            tokens.push(Token::OPERATOR(OPERATOR::PLUS));
        }
        else if text[cursor] == '-' as u8
        {
            tokens.push(Token::OPERATOR(OPERATOR::MINUS));
        }
        else if text[cursor] == '*' as u8
        {
            tokens.push(Token::OPERATOR(OPERATOR::MULTPLY));
        }
        else if text[cursor] == '/' as u8
        {
            tokens.push(Token::OPERATOR(OPERATOR::DIVIDE));
        }
        cursor += 1;
    }
    return vec![Token::NUM(NUMBER::FLOAT(10.0))]
}