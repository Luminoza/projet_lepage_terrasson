use rand::Rng;

#[derive(Clone)]
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub position: (usize, usize),
}

impl Enemy {
    pub fn random(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        Enemy {
            name: "Ennemi".to_string(),
            hp: rng.gen_range(30..50),
            attack: rng.gen_range(5..15),
            position: (x, y),
        }
    }
}

#[derive(Clone)]
pub enum ItemType {
    Potion,
    Equipment(EquipmentType),
}

#[derive(Clone, PartialEq)]
pub enum EquipmentType {
    Hat,
    Vest,
    Pants,
    Shoes,
    Whip,
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub position: (usize, usize),
}

impl Item {
    pub fn random(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        let item_type = if rng.gen_bool(0.5) {
            ItemType::Potion
        } else {
            let equipment_type = match rng.gen_range(0..5) {
                0 => EquipmentType::Hat,
                1 => EquipmentType::Vest,
                2 => EquipmentType::Pants,
                3 => EquipmentType::Shoes,
                _ => EquipmentType::Whip,
            };
            ItemType::Equipment(equipment_type)
        };
        let name = match &item_type {
            ItemType::Potion => "Potion".to_string(),
            ItemType::Equipment(e) => match e {
                EquipmentType::Hat => "Chapeau".to_string(),
                EquipmentType::Vest => "Veste".to_string(),
                EquipmentType::Pants => "Pantalon".to_string(),
                EquipmentType::Shoes => "Chaussures".to_string(),
                EquipmentType::Whip => "Fouet".to_string(),
            },
        };
        Item {
            name,
            item_type,
            position: (x, y),
        }
    }
}

pub struct Player {
    pub name: String,
    pub hp: i32,
    pub inventory: Vec<Item>,
    pub equipment: Vec<EquipmentType>,
}

impl Player {
    pub fn new(name: String, hp: i32) -> Self {
        Player {
            name,
            hp,
            inventory: vec![],
            equipment: vec![],
        }
    }

    pub fn restore_health(&mut self, amount: i32) {
        self.hp += amount;
    }

    pub fn pick_item(&mut self, item: Item) {
        match item.item_type {
            ItemType::Potion => self.inventory.push(item),
            ItemType::Equipment(e) => self.equip(e),
        }
    }

    pub fn equip(&mut self, equipment: EquipmentType) {
        self.equipment.push(equipment);
    }

    pub fn use_potion(&mut self) {
        if let Some(pos) = self.inventory.iter().position(|i| matches!(i.item_type, ItemType::Potion)) {
            self.restore_health(10);
            self.inventory.remove(pos);
        } else {
            println!("Vous n'avez pas de potion !");
        }
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn get_attack_bonus(&self) -> i32 {
        self.equipment.iter().filter(|e| **e == EquipmentType::Whip).count() as i32 * 5
    }

    pub fn get_defense_bonus(&self) -> i32 {
        self.equipment.iter().map(|e| match e {
            EquipmentType::Vest => 3,
            EquipmentType::Pants => 2,
            EquipmentType::Shoes => 1,
            _ => 0,
        }).sum()
    }

    pub fn has_hat(&self) -> bool {
        self.equipment.iter().any(|e| *e == EquipmentType::Hat)
    }
}
