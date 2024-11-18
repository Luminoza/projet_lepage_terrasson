use super::equipment::{EquipmentType, GetName, GetIcon, GetDescription, GetPosition, GetType};

const NAME: &str = "Pants";
const ICON: &str = "👖";
const DESCRIPTION: &str = "Pants provide protection and style.";

pub struct Pants {
    position: (usize, usize),
}

impl Pants {
    pub fn new(position: (usize, usize)) -> Pants {
        Pants {
            position,
        }
    }
}

impl GetName for Pants {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
}

impl GetIcon for Pants {
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
}

impl GetDescription for Pants {
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
}

impl GetPosition for Pants {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
}

impl GetType for Pants {
    fn get_type(&self) -> EquipmentType {
        EquipmentType::Pants
    }
}
