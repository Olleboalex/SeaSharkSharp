use crate::Tokens::*;

fn arithmatic_parser_plus_minus(tokens:Vec<Token>) -> Token
{
    let mut result = Token::token_to_f64(tokens[0].clone());
    let mut cursor = 1;
    let mut is_float = false;
    while cursor < tokens.len()
    {
        match &tokens[cursor]
        {
            Token::OPERATOR(op) =>
            {
                match op
                {
                    OPERATOR::PLUS =>
                    {
                        result += Token::token_to_f64(tokens[cursor + 1].clone());
                    }
                    OPERATOR::MINUS =>
                    {
                        result -= Token::token_to_f64(tokens[cursor + 1].clone());
                    }
                    _ =>
                    {
                        panic!("Incorrect parsing");
                    }
                }
            }
            _ => 
            {
                if !is_float
                {
                    if let Token::NUM(x) = tokens[cursor].clone()
                    {
                        if let NUMBER::FLOAT(_) = x
                        {
                            is_float = true;
                        }
                    }
                }
            }
        }
        cursor += 1;
    }

    if is_float {Token::new_float(result)} else {Token::new_int(result as i32)}
}

fn arithmatic_parser_multiply_divide(tokens:Vec<Token>) -> Token
{
    let mut result = vec![tokens[0].clone()];
    let mut cursor = 1;
    while cursor < tokens.len()
    {
        let resLen = result.len();
        match &tokens[cursor]
        {
            Token::OPERATOR(op) =>
            {
                match op
                {
                    OPERATOR::MULTPLY =>
                    {
                        result[resLen - 1] = Token::multiply_tokens(result[resLen - 1].clone(), tokens[cursor + 1].clone());
                        cursor += 1;
                    }
                    OPERATOR::DIVIDE =>
                    {
                        result[resLen - 1] = Token::divide_tokens(result[resLen - 1].clone(), tokens[cursor + 1].clone());
                        cursor += 1;
                    }
                    _ =>
                    {
                        result.push(tokens[cursor].clone());
                    }
                }
            }
            _ => 
            {
                result.push(tokens[cursor].clone());
            }
        }
        cursor += 1;
    }
    arithmatic_parser_plus_minus(result)
}

fn arithmatic_parser_modulus(tokens:Vec<Token>) -> Token
{
    let mut result = vec![tokens[0].clone()];
    let mut cursor = 1;
    while cursor < tokens.len()
    {
        let resLen = result.len();
        match &tokens[cursor]
        {
            Token::OPERATOR(op) =>
            {
                match op
                {
                    OPERATOR::MODULUS =>
                    {
                        result[resLen - 1] = Token::modulus_tokens(result[resLen - 1].clone(), tokens[cursor + 1].clone());
                        cursor += 1;
                    }
                    _ =>
                    {
                        result.push(tokens[cursor].clone());
                    }
                }
            }
            _ => 
            {
                result.push(tokens[cursor].clone());
            }
        }
        cursor += 1;
    }
    arithmatic_parser_multiply_divide(result)
}

pub fn parse_arithmatic(tokens:Vec<Token>) -> Token
{
    let mut cursor = 0;
    let mut result:Vec<Token> = vec![];
    while cursor < tokens.len()
    {
        match &tokens[cursor]
        {
            Token::OPERATOR(x) =>
            {
                match x
                {
                    OPERATOR::PARAN(y) =>
                    {
                        result.push(parse_arithmatic((*y).clone()));
                    }
                    _ => result.push(tokens[cursor].clone())
                }
            }
            _ => result.push(tokens[cursor].clone())
        }
        cursor += 1;
    }
    arithmatic_parser_modulus(result)
}