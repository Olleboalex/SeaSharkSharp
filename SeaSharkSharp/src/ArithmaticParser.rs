use crate::Tokens::*;

fn ArihmaticParserPlusMinus(tokens:Vec<Token>) -> Token
{
    let mut result = Token::token_to_f64(tokens[0]);
    let mut cursor = 1;
    while cursor < tokens.len()
    {
        match tokens[cursor]
        {
            Token::OPERATOR(op) =>
            {
                match op
                {
                    OPERATOR::PLUS =>
                    {
                        result += Token::token_to_f64(tokens[cursor + 1]);
                    }
                    OPERATOR::MINUS =>
                    {
                        result -= Token::token_to_f64(tokens[cursor + 1]);
                    }
                    _ =>
                    {
                        panic!("Incorrect parsing");
                    }
                }
            }
            _ => 
            {
                panic!("Incorrect parsing");
            }
        }
        cursor += 1;
    }
    Token::new_int(0)
}