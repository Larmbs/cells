use macroquad::prelude::Vec2;

/* Represents a node which other things can connect to */
#[derive(Clone, PartialEq)]
pub struct Node {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub node_type: NodeType,
}

#[derive(Clone, PartialEq)]
pub enum NodeType {
    Fixed,
    Joint,
}
impl Node {
    pub fn new(pos: Vec2, node_type: NodeType) -> Self {
        Self {
            pos,
            prev_pos: pos,
            node_type,
        }
    }
}

/* Represents a connection between nodes */
#[derive(Clone, PartialEq)]
pub struct Rod {
    pub node_a: usize,
    pub node_b: usize,
    pub rod_type: RodType,
}
#[derive(Clone, PartialEq)]
pub enum RodType {
    SOLID {
        length: f32,
    },
    ROPE {
        length: f32,
    },
    SPRING {},
    PISTON {
        min_length: f32,
        max_length: f32,
        length: f32,
    },
}
impl Rod {
    pub fn new(node_a: usize, node_b: usize, rod_type: RodType) -> Self {
        Self {
            node_a,
            node_b,
            rod_type,
        }
    }
}

/* Represents a triangle between three nodes.*/
#[derive(Clone, PartialEq)]
pub struct Triangle {
    node_a: usize,
    node_b: usize,
    node_c: usize,
}
impl Triangle {
    pub fn new(node_a: usize, node_b: usize, node_c: usize) -> Self {
        Self {
            node_a,
            node_b,
            node_c,
        }
    }
}
