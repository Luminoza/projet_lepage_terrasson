use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum EquipmentType {
    Hat,
    Glasses,
    Vest,
    Pants,
    Shoes,
    Whip,
}

#[derive(Deserialize)]
struct EquipmentData {
    name: String,
    icon: String,
    description: String,
}
#[derive(Debug)]
pub struct Equipment {
    name: String,
    icon: String,
    description: String,
    equipment_type: EquipmentType,
    position: (usize, usize),
}

const FILE_PATH: &str = "./src/equipments/equipment_data.json";

impl Equipment {
    pub fn new(equipment_type: EquipmentType, position: (usize, usize)) -> Equipment {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        let equipment_map: HashMap<EquipmentType, EquipmentData> = serde_json::from_str(&data).expect("JSON was not well-formatted");
        let equipment_data = equipment_map.get(&equipment_type).expect("Equipment type not found");

        Equipment {
            name: equipment_data.name.clone(),
            icon: equipment_data.icon.clone(),
            description: equipment_data.description.clone(),
            equipment_type,
            position,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_icon(&self) -> char {
        self.icon.chars().next().unwrap()
    }

    pub fn get_description(&self) -> String {
        self.description.to_string()
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    pub fn get_type(&self) -> EquipmentType {
        self.equipment_type.clone()
    }
}