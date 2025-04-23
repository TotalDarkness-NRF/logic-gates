#[derive(Clone, Debug, PartialEq)]
pub enum LogicGate {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Xor,
    Xnor,
}

impl LogicGate {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogicGate::And => "AND",
            LogicGate::Or => "OR",
            LogicGate::Not => "NOT",
            LogicGate::Nand => "NAND",
            LogicGate::Nor => "NOR",
            LogicGate::Xor => "XOR",
            LogicGate::Xnor => "XNOR",
        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "AND" => Some(LogicGate::And),
            "OR" => Some(LogicGate::Or),
            "NOT" => Some(LogicGate::Not),
            "NAND" => Some(LogicGate::Nand),
            "NOR" => Some(LogicGate::Nor),
            "XOR" => Some(LogicGate::Xor),
            "XNOR" => Some(LogicGate::Xnor),
            _ => None,
        }
    }

    pub fn get_all_gates() -> Vec<LogicGate> {
        return vec![
        LogicGate::And,
        LogicGate::Or,
        LogicGate::Not,
        LogicGate::Nand,
        LogicGate::Nor,
        LogicGate::Xor,
        LogicGate::Xnor,
        ];
    }
}