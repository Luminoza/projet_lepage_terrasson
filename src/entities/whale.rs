use super::entity::{Entity, EntityType, SpecificEntity};

pub struct Whale {
    base: Entity,
}

impl Whale {
    pub fn new(position: (usize, usize)) -> Whale {
        Whale {
            base: Entity::new(EntityType::Whale, position),
        }
    }
}

impl SpecificEntity for Whale {
    fn get_health(&self) -> i32 {
        self.base.hp
    }

    fn get_attack(&self) -> i32 {
        self.base.atk
    }

    fn is_hostile(&self) -> bool {
        self.base.hostile
    }

    fn get_name(&self) -> String {
        self.base.get_name()
    }

    fn get_icon(&self) -> char {
        self.base.get_icon()
    }

    fn get_description(&self) -> String {
        self.base.get_description()
    }

    fn get_position(&self) -> (usize, usize) {
        self.base.get_position()
    }

    fn get_type(&self) -> EntityType {
        self.base.get_type()
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.base.set_position(position);
    }

    fn take_damage(&mut self, damage: i32) {
        self.base.take_damage(damage);
    }

    fn attack(&self, target: &mut dyn SpecificEntity) {
        self.base.attack(target);
    }

    fn heal(&mut self, heal: i32) {
        self.base.heal(heal);
    }

    fn buff_attack(&mut self, buff: i32) {
        self.base.buff_attack(buff);
    }
}
