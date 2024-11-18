use super::equipment::{EquipmentType, GetName, GetIcon, GetDescription, GetPosition, GetType};

const NAME: &str = "Mysterious";
const ICON: &str = "🎁";
const DESCRIPTION: &str = "A Mysterious gift from the gods. Will give you a random item.";


pub struct Mysterious {
    position: (usize, usize),
}

impl Mysterious {
    pub fn new(position: (usize, usize)) -> Mysterious {
        Mysterious {
            position,
        }
    }
}

impl GetName for Mysterious {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
}

impl GetIcon for Mysterious {
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
}

impl GetDescription for Mysterious {
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
}

impl GetPosition for Mysterious {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
}

impl GetType for Mysterious {
    fn get_type(&self) -> EquipmentType {
        EquipmentType::Mysterious
    }
}
