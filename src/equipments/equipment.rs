#[derive(Debug, PartialEq)]
pub enum EquipmentType {
    Mysterious,
    Hat,
    Vest,
    Pants,
    Shoes,
    Whip,
}

pub trait Equipment {
    fn get_name(&self) -> String;
    fn get_icon(&self) -> char;
    fn get_description(&self) -> String;
    fn get_position(&self) -> (usize, usize);
    fn get_type(&self) -> EquipmentType;
}