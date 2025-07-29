//! Vehicle Toolkit

use std::io;
use std::path::PathBuf;
use std::{collections::HashSet, fs::File};

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
use components::*;

/// A craft within a world represents by and organized structure of rods interlocked using nodes
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Craft {
    pub nodes: Vec<Node>,
    pub rods: Vec<Rod>,
    pub triangles: Vec<Triangle>,
}
impl Craft {
    /// Creates a blank craft
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            rods: vec![],
            triangles: vec![],
        }
    }

    /// Removes duplicate parts, tris, rods, and any unused nodes
    pub fn clean_craft(&mut self) {
        // Removing duplicate rods
        let mut seen = HashSet::new();
        self.rods.retain(|p| seen.insert([p.node_a, p.node_b]));

        // Removing duplicate triangles
        let mut seen = HashSet::new();
        self.triangles
            .retain(|p| seen.insert([p.node_a, p.node_b, p.node_c]));
    }

    pub fn node_distance(&self, node_a: usize, node_b: usize) -> f32 {
        self.nodes[node_a].pos.distance(self.nodes[node_b].pos)
    }

    pub fn draw(&self) {
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
            let color = match node.node_type {
                NodeType::Fixed => GREEN,
                NodeType::Joint => BLACK,
            };

            draw_circle(node.pos.x, node.pos.y, 6.0, color);
        }

        // Draw triangles
        for tri in &self.triangles {
            draw_triangle(
                self.nodes[tri.node_a].pos,
                self.nodes[tri.node_b].pos,
                self.nodes[tri.node_c].pos,
                tri.color,
            );
        }
    }
}

impl Craft {
    /// Loads craft from a JSON file
    pub fn load(file_path: PathBuf) -> io::Result<Craft> {
        let file = File::open(file_path)?;
        serde_json::from_reader(file).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "JSON Craft format seems to be invalid",
            )
        })
    }

    /// Saves craft into JSON file
    pub fn save(&self, file_path: PathBuf) -> io::Result<()> {
        let file = File::open(file_path)?;
        serde_json::to_writer(file, self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, "Error saving Craft"));
        io::Result::Ok(())
    }
}
