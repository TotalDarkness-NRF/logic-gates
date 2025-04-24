use super::gate_type::GateType;

#[derive(Clone, Debug, PartialEq)]
pub struct LogicGate {
    pub gate_type: GateType,
    pos: Option<(i32, i32)>,
    id: String,
}

impl LogicGate {
    pub fn get_logic_gate(gate_type: GateType) -> Self {
        Self { gate_type, pos: None, id: uuid::Uuid::new_v4().to_string()}
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_x(&self) -> Option<i32> {
        if let Some(pos) = self.pos {
            Some(pos.0)
        } else { None }
    }

    pub fn get_y(&self) -> Option<i32> {
        if let Some(pos) = self.pos {
            Some(pos.1)
        } else { None }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = Some((x, y));
    }
}