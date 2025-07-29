//! Vehicle Toolkit
//!
//! This file represents the components the make up a craft
//!
//! Nodes:
//!
//! Rods:
//!     This part is the bread and butter of engineering any craft.
//!     It connects all the nodes, and parts together making up the skeleton of every craft
//!     Each rod has settings to change their behavior like their stretch and strength
//!
//! Parts:
//!     These are special components on craft that have a special functionality
//!     Each part comes with attachment points and some means of activation,
//!     so you can strap on engine or wheels to your vehicle to bring in functionality
//!
//! Triangles:
//!     Quick and easy way of filling in panels on a craft to give it a smoother design
//!
use macroquad::prelude::{Color, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vec2")]
struct Vec2Def {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
struct ColorDef {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

/* Represents a node which other things can connect to */
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Node {
    #[serde(with = "Vec2Def")]
    pub pos: Vec2,
    #[serde(with = "Vec2Def")]
    pub prev_pos: Vec2,
    pub node_type: NodeType,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum NodeType {
    Fixed,
    Joint,
}
impl Default for NodeType {
    fn default() -> Self {
        NodeType::Joint
    }
}

/* Represents a connection between nodes */
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Rod {
    pub node_a: usize,
    pub node_b: usize,
    pub length: f32,
    pub rod_type: RodType,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum RodType {
    SOLID,
    ROPE,
    SPRING,
    PISTON { min_length: f32, max_length: f32 },
}
impl Default for RodType {
    fn default() -> Self {
        Self::SOLID
    }
}

/* Part */
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Part {
    Wheel {
        #[serde(with = "Vec2Def")]
        pos: Vec2,
        #[serde(with = "Vec2Def")]
        prev_pos: Vec2,
        wheel_radius: f32,
    },
    JetEngine {
        #[serde(with = "Vec2Def")]
        pos: Vec2,
        #[serde(with = "Vec2Def")]
        prev_pos: Vec2,
        angle: f32,
    },
}

/* Represents a triangle between three nodes.*/
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Triangle {
    pub node_a: usize,
    pub node_b: usize,
    pub node_c: usize,
    #[serde(with = "ColorDef")]
    pub color: Color,
}
