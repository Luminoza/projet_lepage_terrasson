use super::equipment::{EquipmentType, GetName, GetIcon, GetDescription, GetPosition, GetType};

const NAME: &str = "Shoes";
const ICON: &str = "👟";
const DESCRIPTION: &str = "Shoes allow you to move faster and more comfortably.";

pub struct Shoes {
    position: (usize, usize),
}

impl Shoes {
    pub fn new(position: (usize, usize)) -> Shoes {
        Shoes {
            position,
        }
    }
}

impl GetName for Shoes {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
}

impl GetIcon for Shoes {
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
}

impl GetDescription for Shoes {
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
}

impl GetPosition for Shoes {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
}

impl GetType for Shoes {
    fn get_type(&self) -> EquipmentType {
        EquipmentType::Shoes
    }
}
