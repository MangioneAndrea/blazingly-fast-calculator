use crate::generic_error::ParsingTokenError;

#[derive(PartialEq, Clone, Debug)]
pub enum UnaryOp {
    SquareRoot,
    Cos,
    Acos,
    Sin,
    Log10
}

impl UnaryOp {
    pub fn exec(&self, a: f32) -> f32 {
        match self {
            Self::SquareRoot => a.sqrt(),
            Self::Cos => a.cos(),
            Self::Acos => a.acos(),
            Self::Sin => a.sin(),
            Self::Log10 => a.log10(),
        }
    }
}

impl TryFrom<&str> for UnaryOp {
    type Error = ParsingTokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "√" => Ok(Self::SquareRoot),
            "cos" => Ok(Self::Cos),
            "acos" => Ok(Self::Acos),
            "sin" => Ok(Self::Sin),
            "log" => Ok(Self::Log10),
            _ => Err(ParsingTokenError::OperationNotImplemented),
        }
    }
}
