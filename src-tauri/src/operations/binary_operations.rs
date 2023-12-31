use crate::generic_error::ParsingTokenError;

#[derive(PartialEq, Clone, Debug)]
pub enum BinaryOp {
    Sum,
    Sub,
    Mul,
    Div,
    Pow,
}

impl BinaryOp {
    pub fn exec(&self, a: f32, b: f32) -> f32 {
        match self {
            Self::Sum => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
            Self::Pow => a.powf(b),
        }
    }
}

impl TryFrom<&str> for BinaryOp {
    type Error = ParsingTokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Sum),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "^" => Ok(Self::Pow),
            _ => Err(ParsingTokenError::OperationNotImplemented),
        }
    }
}
