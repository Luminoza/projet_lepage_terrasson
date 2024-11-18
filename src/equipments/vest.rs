use super::equipment::{EquipmentType, Equipment};

const NAME: &str = "Vest";
const ICON: &str = "🦺";
const DESCRIPTION: &str = "A vest provides protection and warmth.";
const TYPE: EquipmentType = EquipmentType::Vest;

pub struct Vest {
    position: (usize, usize),
}

impl Equipment for Vest {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
    fn get_type(&self) -> EquipmentType {
        TYPE
    }
}

impl Vest {
    pub fn new(position: (usize, usize)) -> Vest {
        Vest {
            position,
        }
    }
}
