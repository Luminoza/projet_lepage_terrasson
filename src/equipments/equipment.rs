use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use rand::Rng; // Add this import

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
#[derive(Debug, Clone)]
pub struct Equipment {
    name: String,
    icon: String,
    description: String,
    equipment_type: EquipmentType,
    position: (usize, usize),
    visible: bool,
    equiped: bool,
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
            visible: false,
            equiped: false,
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

    pub fn random() -> EquipmentType {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => EquipmentType::Hat,
            1 => EquipmentType::Glasses,
            2 => EquipmentType::Vest,
            3 => EquipmentType::Pants,
            4 => EquipmentType::Shoes,
            5 => EquipmentType::Whip,
            _ => EquipmentType::Pants,
       }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_equiped(&self) -> bool {
        self.equiped
    }

    pub fn set_equiped(&mut self, equiped: bool) {
        self.equiped = equiped;
    }
}