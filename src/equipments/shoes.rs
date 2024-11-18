use super::equipment::{EquipmentType, Equipment};

const NAME: &str = "Shoes";
const ICON: &str = "👟";
const DESCRIPTION: &str = "Shoes allow you to move faster and more comfortably.";
const TYPE: EquipmentType = EquipmentType::Shoes;

pub struct Shoes {
    position: (usize, usize),
}

impl Equipment for Shoes {
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

impl Shoes {
    pub fn new(position: (usize, usize)) -> Shoes {
        Shoes {
            position,
        }
    }
}
