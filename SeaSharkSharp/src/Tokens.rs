#[derive(Clone, Copy)]
pub enum NUMBER
{
    FLOAT(f64),
    INT(i32)
}
#[derive(Clone, Copy)]
pub enum OPERATOR
{
    PLUS,
    MINUS,
    MULTPLY,
    DIVIDE,
    MODULUS
}
#[derive(Clone, Copy)]
pub enum Token
{
    NUM(NUMBER),
    OPERATOR(OPERATOR)
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