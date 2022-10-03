use crate::Tokens::*;

fn KeyWordFinder(text:&[u8], pos:usize, find:String) -> bool
{
    false
}

fn IsNumber(x:u8) -> bool
{
    x >= '0' as u8 && x <= '9' as u8
}

pub fn lex_text(input:String) -> Vec<Token>
{
    let text:&[u8] = input.as_bytes();
    let mut tokens:Vec<Token> = vec![];
    let mut cursor = 0;
    while cursor < text.len()
    {
        if text[cursor] != ' ' as u8 && text[cursor] != '\n' as u8
        {
            if IsNumber(text[cursor])
            {
                let mut temp:Vec<u8> = vec![];
                while cursor < text.len()
                {
                    if !IsNumber(text[cursor]) && text[cursor] != '.' as u8
                    {
                        cursor -= 1;
                        break;
                    }
                    temp.push(text[cursor]);
                    cursor += 1;
                }
                tokens.push(Token::parse_number(temp));
            }
            else if text[cursor] == '+' as u8
            {
                tokens.push(Token::OPERATOR(OPERATOR::PLUS));
            }
            else if text[cursor] == '-' as u8
            {
                if IsNumber(text[cursor + 1])
                {
                    cursor += 1;
                    let mut temp:Vec<u8> = vec!['-' as u8];
                    while cursor < text.len()
                    {
                        if !IsNumber(text[cursor]) && text[cursor] != '.' as u8
                        {
                            cursor -= 1;
                            break;
                        }
                        temp.push(text[cursor]);
                        cursor += 1;
                    }
                    tokens.push(Token::parse_number(temp));
                }
                else
                {
                    tokens.push(Token::OPERATOR(OPERATOR::MINUS));
                }
            }
            else if text[cursor] == '*' as u8
            {
                tokens.push(Token::OPERATOR(OPERATOR::MULTPLY));
            }
            else if text[cursor] == '/' as u8
            {
                tokens.push(Token::OPERATOR(OPERATOR::DIVIDE));
            }
            else if text[cursor] == '%' as u8
            {
                tokens.push(Token::OPERATOR(OPERATOR::MODULUS));
            }
        }
        cursor += 1;
    }
    tokens
}