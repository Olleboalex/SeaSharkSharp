pub enum NUMBER
{
    FLOAT(f64),
    INT(i32)
}
pub enum OPERATOR
{
    PLUS,
    MINUS,
    MULTPLY,
    DIVIDE,
    MODULUS
}
pub enum Token
{
    NUM(NUMBER),
    OPERATOR(OPERATOR)
}

impl Token
{
    pub fn newInt(x:i32) -> Token
    {
        Token::NUM(NUMBER::INT(x))
    }
    pub fn newFloat(x:f64) -> Token
    {
        Token::NUM(NUMBER::FLOAT(x))
    }
    pub fn ParseNumber(text:Vec<u8>) -> Token
    {
        Token::newInt(0)
    }
}