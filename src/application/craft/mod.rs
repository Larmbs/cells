//! Represent what a craft is, different components and functions that are customizable in the craft

use macroquad::prelude::*;

pub mod components;
use components::*;

/// A craft within a world represents by and organized structure of rods interlocked using nodes
#[derive(Clone, PartialEq)]
pub struct Craft {
    pub nodes: Vec<Node>,
    pub rods: Vec<Rod>,
    pub triangles: Vec<Triangle>,
}
impl Craft {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            rods: vec![],
            triangles: vec![],
        }
    }

    pub fn draw(&self, highlighted_nodes: &Vec<usize>) {
        // Draw rods
        for rod in &self.rods {
            let (a, b) = (rod.node_a, rod.node_b);
            let color = match rod.rod_type {
                RodType::SOLID { .. } => DARKGRAY,
                RodType::ROPE { .. } => RED,
                RodType::SPRING {} => BLUE,
                RodType::PISTON { .. } => ORANGE,
            };
            draw_line(
                self.nodes[a].pos.x,
                self.nodes[a].pos.y,
                self.nodes[b].pos.x,
                self.nodes[b].pos.y,
                2.0,
                color,
            );
        }

        // Draw nodes
        for (node, i) in self.nodes.iter().zip(0..self.nodes.len()) {
            let color = if highlighted_nodes.contains(&i) {
                SKYBLUE
            } else {
                match node.node_type {
                    NodeType::Fixed => GREEN,
                    NodeType::Joint => BLACK,
                }
            };
            draw_circle(node.pos.x, node.pos.y, 6.0, color);
        }

        // Draw floor
        draw_line(0.0, 600.0, screen_width(), 600.0, 2.0, GREEN);
    }
}
