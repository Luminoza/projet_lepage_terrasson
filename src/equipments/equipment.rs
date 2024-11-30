use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use rand::Rng; // Add this import

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]

/**
 * Enumération des types d'équipements
 */
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

    /**
     * Crée un nouvel équipement
     */
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
            visible: true,
            equiped: false,
        }
    }

    /**
     * Retourne le nom de l'équipement
     */
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /**
     * Retourne l'icone de l'équipement
     */
    pub fn get_icon(&self) -> &str {
        self.icon.as_str()
    }

    /**
     * Retourne la description de l'équipement
     */
    pub fn get_description(&self) -> String {
        self.description.to_string()
    }

    /**
     * Retourne la position de l'équipement
     */
    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    /**
     * Retourne le type de l'équipement
     */
    pub fn get_type(&self) -> EquipmentType {
        self.equipment_type.clone()
    }

    /**
     * Retourne un type d'équipement aléatoire
     */
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

    /**
     * Retourne si l'équipement est visible
     */
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /**
     * Définit si l'équipement est visible
     */
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /**
     * Retourne si l'équipement est équipé
     */
    pub fn is_equiped(&self) -> bool {
        self.equiped
    }

    /**
     * Définit si l'équipement est équipé
     */
    pub fn set_equiped(&mut self, equiped: bool) {
        self.equiped = equiped;
    }
}


pub struct EquipmentManager {
    equipments: Vec<Equipment>,
}

impl EquipmentManager {

    /**
     * Crée un nouveau gestionnaire d'équipements
     */
    pub fn new() -> EquipmentManager {
        EquipmentManager {
            equipments: Vec::new(),
        }
    }

    /**
     * Ajoute un équipement
     */
    pub fn add(&mut self, equipment: Equipment) {
        self.equipments.push(equipment);
    }

    /**
     * Retourne les équipements
     */
    pub fn within_range(&self, position: (usize, usize), range: usize) -> Vec<&Equipment> {
        self.equipments.iter().filter(|equipment| {
            let (x, y) = equipment.get_position();
            let (x, y) = (x , y );
            let (px, py) = (position.0 , position.1 );
            ((px as isize - x as isize).abs() as usize) <= range && ((py as isize - y as isize).abs() as usize) <= range
        }).collect()
    }

    /**
     * Retourne si une position est occupée par un équipement
     */
    pub fn is_position_occupied(&self, position: (usize, usize)) -> bool {
        self.equipments.iter().any(|equipment| equipment.get_position() == position)
    }

    /**
     * Retourne un équipement à une position donnée
     */
    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut Equipment> {
        for equipment in &mut self.equipments {
            if equipment.get_position() == position {
                return Some(equipment);
            }
        }
        None
    }
    
}