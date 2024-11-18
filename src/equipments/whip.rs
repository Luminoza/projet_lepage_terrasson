use super::equipment::{EquipmentType, Equipment};

const NAME: &str = "Whip";
const ICON: &str = "🔫";
const DESCRIPTION: &str = "A whip allows you to inflict more damage.";
const TYPE: EquipmentType = EquipmentType::Whip;

pub struct Whip {
    position: (usize, usize),
}

impl Equipment for Whip {
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

impl Whip {
    pub fn new(position: (usize, usize)) -> Whip {
        Whip {
            position,
        }
    }
}
