use std::collections::HashMap;

pub type PositionUpdateResource = HashMap<u16, PositionUpdate>;

pub struct PositionUpdate {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
}
