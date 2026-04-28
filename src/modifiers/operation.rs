/// Component-wise operation applied by modifiers.
///
/// `Operation` describes how a nested modifier's result should combine with
/// the base point value. `None` indicates the modifier replaces the value.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operation {
    /// No operation — the modifier's value replaces the base value.
    None = 0,
    /// Add the modifier value to the base value.
    Add,
    /// Subtract the modifier value from the base value.
    Sub,
    /// Multiply the base value by the modifier value.
    Mul,
    /// Divide the base value by the modifier value.
    Div,
}

impl std::str::FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opAdd" => Ok(Self::Add),
            "opSub" => Ok(Self::Sub),
            "opMul" => Ok(Self::Mul),
            "opDiv" => Ok(Self::Div),
            _ => Ok(Self::None),
        }
    }
}
