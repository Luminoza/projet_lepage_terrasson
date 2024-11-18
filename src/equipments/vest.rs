use super::equipment::{EquipmentType, GetName, GetIcon, GetDescription, GetPosition, GetType};

const NAME: &str = "Vest";
const ICON: &str = "🦺";
const DESCRIPTION: &str = "A vest provides protection and warmth.";

pub struct Vest {
    position: (usize, usize),
}

impl Vest {
    pub fn new(position: (usize, usize)) -> Vest {
        Vest {
            position,
        }
    }
}

impl GetName for Vest {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
}

impl GetIcon for Vest {
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
}

impl GetDescription for Vest {
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
}

impl GetPosition for Vest {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
}

impl GetType for Vest {
    fn get_type(&self) -> EquipmentType {
        EquipmentType::Vest
    }
}
