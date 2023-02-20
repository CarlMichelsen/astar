use std::collections::HashMap;

use uuid::Uuid;

#[derive(Clone)]
pub struct Nodeset {
    pub name: String,
    pub nodes: HashMap<Uuid, Node>,
}

#[derive(Clone)]
pub struct Node {
    pub id: Uuid,
    pub links: Vec<Uuid>,
    pub position: Position,
}

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/*impl Position {
    fn add(&self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn subtract(&self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(&self, scalar: f32) -> Position {
        Position {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn dot(&self, other: &Position) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Position) -> Position {
        Position {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}*/
