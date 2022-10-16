use std::collections::{hash_set, HashSet};

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
    let MATH_TOKENS:HashSet<u8> = vec!('+' as u8, '-' as u8, '*' as u8, '/' as u8, '%' as u8, '(' as u8, ')' as u8, '=' as u8).into_iter().collect();

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
            else if KeyWordFinder(text, cursor, String::from("while"))
            {
                cursor += 5;
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
                    panic!("No starting parenthesis to while loop");
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
                    panic!("No end to while loop condition");
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
                    panic!("No beginning to while loop");
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
                    panic!("No ending to while loop");
                }
                tokens.push(Token::WHILE(lex_text(u8_vec_to_string(condition_text)), lex_text(u8_vec_to_string(statement_text))))
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
            else
            {
                let mut name:Vec<u8> = vec![];
                while cursor < text.len()
                {
                    if MATH_TOKENS.contains(&text[cursor]) || text[cursor] == ' ' as u8 || text[cursor] == '\n' as u8
                    {
                        break;
                    }
                    name.push(text[cursor]);
                    cursor += 1;
                }
                if text[cursor] == '(' as u8
                {
                    let mut LPar = 1;
                    let mut LBrack = 0;
                    let mut QuoteMark = 0;

                    let mut parameters:Vec<String> = vec![];
                    let mut temp:Vec<u8> = vec![];

                    while cursor < text.len()
                    {
                        if text[cursor] == '(' as u8
                        {
                            LPar += 1;
                        }
                        else if text[cursor] == ')' as u8
                        {
                            LPar -= 1;
                        }
                        else if text[cursor] == '[' as u8
                        {
                            LBrack += 1;
                        }
                        else if text[cursor] == ']' as u8
                        {
                            LBrack -= 1;
                        }
                        else if text[cursor] == '"' as u8 && text[cursor - 1] != '\\' as u8
                        {
                            QuoteMark += 1;
                        }
                        if LPar == 0
                        {
                            break;
                        }
                        if text[cursor] == ',' as u8 && LPar == 1 && LBrack == 0 && QuoteMark % 2 == 0
                        {
                            parameters.push(u8_vec_to_string(temp.clone()));
                            temp.clear();
                        }
                        else
                        {
                            temp.push(text[cursor]);
                        }
                        cursor += 1;
                    }

                    if text[cursor] != ')' as u8
                    {
                        panic!("No ending to parameters in function call");
                    }
                    if temp.len() > 0
                    {
                        parameters.push(u8_vec_to_string(temp.clone()));
                    }
                    let mut lexed_parameters:Vec<Vec<Token>> = vec![];
                    for element in parameters
                    {
                        lexed_parameters.push(lex_text(element));
                    }
                    tokens.push(Token::METHODCALL(u8_vec_to_string(name), lexed_parameters));
                }
                else
                {
                    cursor -= 1;
                    tokens.push(Token::VARIABLE(u8_vec_to_string(name)));
                }
            }
        }
        cursor += 1;
    }
    tokens
}