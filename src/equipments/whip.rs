use super::equipment::{EquipmentType, GetName, GetIcon, GetDescription, GetPosition, GetType};

const NAME: &str = "Whip";
const ICON: &str = "🔗";
const DESCRIPTION: &str = "A whip allows you to inflict more damage.";

pub struct Whip {
    position: (usize, usize),
}

impl Whip {
    pub fn new(position: (usize, usize)) -> Whip {
        Whip {
            position,
        }
    }
}

impl GetName for Whip {
    fn get_name(&self) -> String {
        NAME.to_string()
    }
}

impl GetIcon for Whip {
    fn get_icon(&self) -> char {
        ICON.chars().next().unwrap()
    }
}

impl GetDescription for Whip {
    fn get_description(&self) -> String {
        DESCRIPTION.to_string()
    }
}

impl GetPosition for Whip {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }
}

impl GetType for Whip {
    fn get_type(&self) -> EquipmentType {
        EquipmentType::Whip
    }
}
