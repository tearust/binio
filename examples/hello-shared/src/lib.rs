use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)] 
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)] 
pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}