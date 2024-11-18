use super::equipment::{EquipmentType, Equipment};

const NAME: &str = "Mysterious";
const ICON: &str = "🎁";
const DESCRIPTION: &str = "A Mysterious gift from the gods. Will give you a random item.";
const TYPE: EquipmentType = EquipmentType::Mysterious;

pub struct Mysterious {
    position: (usize, usize),
}

impl Equipment for Mysterious {
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

impl Mysterious {
    pub fn new(position: (usize, usize)) -> Mysterious {
        Mysterious {
            position,
        }
    }
}
