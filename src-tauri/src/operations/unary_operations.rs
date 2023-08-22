use crate::generic_error::ParsingTokenError;

#[derive(PartialEq, Clone, Debug)]
pub enum UnaryOp {
    SquareRoot,
    Cos,
    Sin,
}

impl UnaryOp {
    pub fn exec(&self, a: f32) -> f32 {
        match self {
            Self::SquareRoot => a.sqrt(),
            Self::Cos => a.cos(),
            Self::Sin => a.sin(),
        }
    }
}

impl TryFrom<&str> for UnaryOp {
    type Error = ParsingTokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "√" => Ok(Self::SquareRoot),
            "cos" => Ok(Self::Cos),
            "sin" => Ok(Self::Sin),
            _ => Err(ParsingTokenError::OperationNotImplemented),
        }
    }
}
