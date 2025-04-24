#[derive(Clone, Debug, PartialEq, Copy)]
pub enum GateType {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Xor,
    Xnor,
}

impl GateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
            Self::Nand => "NAND",
            Self::Nor => "NOR",
            Self::Xor => "XOR",
            Self::Xnor => "XNOR",
        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "AND" => Some(GateType::And),
            "OR" => Some(GateType::Or),
            "NOT" => Some(GateType::Not),
            "NAND" => Some(GateType::Nand),
            "NOR" => Some(GateType::Nor),
            "XOR" => Some(GateType::Xor),
            "XNOR" => Some(GateType::Xnor),
            _ => None,
        }
    }

    pub fn get_all_gates() -> Vec<Self> {
        return vec![
        Self::And,
        Self::Or,
        Self::Not,
        Self::Nand,
        Self::Nor,
        Self::Xor,
        Self::Xnor,
        ];
    }
}