use super::equipment::{EquipmentType, GetName, GetIcon, GetDescription, GetPosition, GetType};

const NAME: &str = "Hat";
const ICON: &str = "🎩";
const DESCRIPTION: &str = "A hat allows you to look cool. And to see enemies from furthur away";


pub struct Hat {
    position: (usize, usize),
}

impl Hat {
    pub fn new(position: (usize, usize)) -> Hat {
        Hat {
            position,
        }
    }
}

impl GetName for Hat {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
}

impl GetIcon for Hat {
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
}

impl GetDescription for Hat {
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
}

impl GetPosition for Hat {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
}

impl GetType for Hat {
    fn get_type(&self) -> EquipmentType {
        EquipmentType::Hat
    }
}
