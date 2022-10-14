use crate::Tokens::*;

fn KeyWordFinder(text:&[u8], pos:usize, find:String) -> bool
{
    let keyword = find.as_bytes();
    let mut i = 0;
    while pos + i < text.len() && i < keyword.len()
    {
        if text[pos + i] != keyword[i]
        {
            return false;
        }
        i += 1;
    }
    return true;
}

fn u8_vec_to_string (input:Vec<u8>) -> String
{
    let temp = String::from_utf8(input);
    match temp
    {
        Result::Ok(x) =>
        {
            x
        }
        _ => panic!("Failed to parse string")
    }
}

fn IsNumber(x:u8) -> bool
{
    x >= '0' as u8 && x <= '9' as u8
}

pub fn print_u8_vec(x:Vec<u8>)
{
    for element in x
    {
        print!("{}", element as char);
    }
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
            else if text[cursor] == '(' as u8
            {
                cursor += 1;
                let mut contained: Vec<u8> = vec![];
                let mut LParan = 1;
                while cursor < text.len()
                {
                    if(text[cursor] == '(' as u8)
                    {
                        LParan += 1;
                    }
                    else if text[cursor] == ')' as u8
                    {
                        LParan -= 1;
                    }
                    if LParan == 0
                    {
                        break;
                    }
                    contained.push(text[cursor]);
                    cursor += 1;
                }
                if text[cursor] != ')' as u8
                {
                    panic!("No end to paranthesis");
                }
                let result = String::from_utf8(contained);
                match result
                {
                    Result::Ok(x) =>
                    {
                        tokens.push(Token::OPERATOR(OPERATOR::PARAN(lex_text(x))))
                    }
                    _ => panic!("Can't parse that")
                }
            }
            else if text[cursor] == ')' as u8
            {
                panic!("No start to parenthesis");
            }
            else if KeyWordFinder(text, cursor, String::from("if"))
            {
                cursor += 2;
                while cursor < text.len()
                {
                    if text[cursor] == '(' as u8
                    {
                        break;
                    }
                    cursor += 1;
                }
                if cursor < text.len() && text[cursor] != '(' as u8
                {
                    panic!("No starting parenthesis to if statement");
                }
                cursor += 1;
                let mut condition_text:Vec<u8> = vec![];
                let mut LParan = 1;
                while cursor < text.len()
                {
                    if text[cursor] == '(' as u8
                    {
                        LParan += 1;
                    }
                    else if text[cursor] == ')' as u8
                    {
                        LParan -= 1;
                    }
                    if LParan == 0
                    {
                        break;
                    }
                    condition_text.push(text[cursor]);
                    cursor += 1;
                }
                if cursor < text.len() && text[cursor] != ')' as u8
                {
                    panic!("No end to if statement condition");
                }
                while cursor < text.len()
                {
                    if text[cursor] == '{' as u8
                    {
                        break;
                    }
                    cursor += 1;
                }
                
                if cursor < text.len() && text[cursor] != '{' as u8
                {
                    panic!("No beginning to if statement");
                }
                let mut statement_text:Vec<u8> = vec![];
                let mut LBrack = 1;
                while cursor < text.len()
                {
                    if text[cursor] == '{' as u8
                    {
                        LBrack += 1;
                    }
                    else if text[cursor] == '}' as u8
                    {
                        LBrack -= 1;
                    }
                    if LBrack == 0
                    {
                        break;
                    }
                    statement_text.push(text[cursor]);
                    cursor += 1;
                }

                if cursor < text.len() && text[cursor] != '}' as u8
                {
                    panic!("No ending to if statement");
                }
                tokens.push(Token::IF(lex_text(u8_vec_to_string(condition_text)), lex_text(u8_vec_to_string(statement_text))))
            }
            else if KeyWordFinder(text, cursor, String::from("true"))
            {
                cursor += 3;
                tokens.push(Token::BOOL(true));
            }
            else if KeyWordFinder(text, cursor, String::from("false"))
            {
                cursor += 4;
                tokens.push(Token::BOOL(false));
            }

        }
        cursor += 1;
    }
    tokens
}