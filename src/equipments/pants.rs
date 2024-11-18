use super::equipment::{EquipmentType, Equipment};

const NAME: &str = "Pants";
const ICON: &str = "👖";
const DESCRIPTION: &str = "Pants provide protection and style.";
const TYPE: EquipmentType = EquipmentType::Pants;

pub struct Pants {
    position: (usize, usize),
}

impl Equipment for Pants {
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

impl Pants {
    pub fn new(position: (usize, usize)) -> Pants {
        Pants {
            position,
        }
    }
}
