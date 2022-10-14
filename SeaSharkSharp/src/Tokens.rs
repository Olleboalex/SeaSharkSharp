#[derive(Clone)]
pub enum NUMBER
{
    FLOAT(f64),
    INT(i32)
}
#[derive(Clone)]
pub enum OPERATOR
{
    PLUS,
    MINUS,
    MULTPLY,
    DIVIDE,
    MODULUS,
    PARAN(Vec<Token>)
}
#[derive(Clone)]
pub enum Token
{
    NUM(NUMBER),
    OPERATOR(OPERATOR),
    BOOL(bool),
    IF(Vec<Token>, Vec<Token>)
}

fn u8_to_i16(x:u8) -> i16
{
    (x - '0' as u8) as i16
}

impl Token
{
    pub fn new_int(x:i32) -> Token
    {
        Token::NUM(NUMBER::INT(x))
    }
    pub fn new_float(x:f64) -> Token
    {
        Token::NUM(NUMBER::FLOAT(x))
    }
    pub fn token_to_f64(x:Token) -> f64
    {
        match x
        {
            Token::NUM(val) =>
            {
                match val
                {
                    NUMBER::FLOAT(y) => y,
                    NUMBER::INT(y) => y as f64
                }
            }
            _ => 0.0
        }
    }
    pub fn multiply_tokens(x:Token, y:Token) -> Token
    {
        let mut result = 0.0;
        let mut is_float = false;
        match x
        {
            Token::NUM(z) =>
            {
                match z
                {
                    NUMBER::FLOAT(w) =>
                    {
                        result = w;
                        is_float = true;
                    }
                    NUMBER::INT(w) =>
                    {
                        result = w as f64;
                    }
                }
            }
            _ => panic!("Multiplying tokens requires numbers!")
        }
        match y
        {
            Token::NUM(z) =>
            {
                match z
                {
                    NUMBER::FLOAT(w) =>
                    {
                        result *= w;
                        is_float = true;
                    }
                    NUMBER::INT(w) =>
                    {
                        result *= w as f64;
                    }
                }
            }
            _ => panic!("Multiplying tokens requires numbers!")
        }

        if is_float {Token::new_float(result)} else {Token::new_int(result as i32)}
    }
    pub fn divide_tokens(x:Token, y:Token) -> Token
    {
        let mut result = 0.0;
        let mut is_float = false;
        match x
        {
            Token::NUM(z) =>
            {
                match z
                {
                    NUMBER::FLOAT(w) =>
                    {
                        result = w;
                        is_float = true;
                    }
                    NUMBER::INT(w) =>
                    {
                        result = w as f64;
                    }
                }
            }
            _ => panic!("Dividing tokens requires numbers!")
        }
        match y
        {
            Token::NUM(z) =>
            {
                match z
                {
                    NUMBER::FLOAT(w) =>
                    {
                        result /= w;
                        is_float = true;
                    }
                    NUMBER::INT(w) =>
                    {
                        result /= w as f64;
                    }
                }
            }
            _ => panic!("Dividing tokens requires numbers!")
        }

        if is_float {Token::new_float(result)} else {Token::new_int(result as i32)}
    }
    pub fn modulus_tokens(x:Token, y:Token) -> Token
    {
        match x
        {
            Token::NUM(z) =>
            {
                match z
                {
                    NUMBER::INT(num1) =>
                    {
                        match y
                        {
                            Token::NUM(w) =>
                            {
                                match w
                                {
                                    NUMBER::INT(num2) =>
                                    {
                                        return Token::new_int(num1 % num2);
                                    }
                                    _ => panic!("Error when doing modulus!")
                                }
                            }
                            _ => panic!("Error when doing modulus!")
                        }
                    }
                    _ => panic!("When doing modulus operator, all numbers must be integers")
                }
            }
            _ => panic!("Error when doing modulus!")
        }
    }
    pub fn parse_number(text:Vec<u8>) -> Token
    {
        let mut Negative = false;
        let mut LSide:Vec<i16> = vec![];
        let mut RSide:Vec<i16> = vec![];

        let mut cursor = 0;
        if text[cursor] == '-' as u8
        {
            Negative = true;
            cursor = 1;
        }
        while cursor < text.len()
        {
            if text[cursor] == '.' as u8
            {
                cursor += 1;
                break;
            }
            LSide.push(u8_to_i16(text[cursor]));
            cursor += 1;
        }
        while cursor < text.len()
        {
            RSide.push(u8_to_i16(text[cursor]));
            cursor += 1;
        }

        let LenL = LSide.len() as i32;
        let LenR = RSide.len() as i32;

        if LenR > 0
        {
            let mut result = 0.0;
            let mut i = 0;
            while i < LenL
            {
                result += (LSide[i as usize] as f64) * f64::powi(10.0, LenL - i - 1);
                i += 1;
            }
            i = 0;
            while i < LenR
            {
                result += (RSide[i as usize] as f64) * f64::powi(10.0, -(i + 1));
                i += 1;
            }
            result = if Negative {-result} else {result};
            return Token::new_float(result);
        }

        let mut result = 0;
        let mut i = 0;
        while i < LenL
        {
            result += (LSide[i as usize] as i32) * i32::pow(10, (LenL - i - 1) as u32);
            i += 1;
        }
        result = if Negative {-result} else {result};
        Token::new_int(result)
    }
}