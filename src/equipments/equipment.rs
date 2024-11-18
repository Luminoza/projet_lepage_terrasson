#[derive(Debug)]
pub enum EquipmentType {
    Mysterious,
    Hat,
    Vest,
    Pants,
    Shoes,
    Whip,
}

pub trait GetName {
    fn get_name(&self) -> String;
}

pub trait GetIcon {
    fn get_icon(&self) -> char;
}

pub trait GetDescription {
    fn get_description(&self) -> String;
}

pub trait GetPosition {
    fn get_position(&self) -> (usize, usize);
}

pub trait GetType {
    fn get_type(&self) -> EquipmentType;
}


