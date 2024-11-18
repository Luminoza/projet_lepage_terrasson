use super::equipment::{EquipmentType, Equipment};

const NAME: &str = "Hat";
const ICON: &str = "🎩";
const DESCRIPTION: &str = "A hat allows you to look cool. And to see enemies from furthur away";
const TYPE: EquipmentType = EquipmentType::Hat;

pub struct Hat {
    position: (usize, usize),
}

impl Equipment for Hat {
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

impl Hat {
    pub fn new(position: (usize, usize)) -> Hat {
        Hat {
            position,
        }
    }
}