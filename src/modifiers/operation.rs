#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operation {
    None = 0,
    Add,
    Sub,
    Mul,
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
