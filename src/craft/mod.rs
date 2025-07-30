//! # Vehicle Toolkit
//!
//! This module provides the `Craft` structure and associated functionality
//! for representing and manipulating a physics-based mechanical structure
//! built using nodes and rods.
//!
//! A `Craft` is composed of:
//! - **Nodes**: connection points that define locations in 2D space.
//! - **Rods**: links between nodes that simulate physical constraints (like solid bars, springs, ropes, etc.).
//!
//! The module supports loading/saving a craft from disk and rendering it visually.
use std::fs::File;
use std::io;
use std::path::PathBuf;

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod editor;
use components::*;

/// Represents a physical structure made of nodes connected by rods.
/// Nodes are points in 2D space, and rods define physical constraints between them.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Craft {
    pub nodes: Vec<Node>,
    pub rods: Vec<Rod>,
}
impl Craft {
    /// Creates a new, empty craft.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            rods: vec![],
        }
    }
}
impl Craft {
    /// Loads craft from a JSON file
    pub fn load(file_path: PathBuf) -> io::Result<Craft> {
        let file = File::open(file_path)?;
        serde_json::from_reader(file).map_err(|_| {
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
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Error saving Craft"))?;
        io::Result::Ok(())
    }
}

/// Temporary, will find a better way to structure project in the future
pub fn draw_craft(craft: &Craft) {
    // Draw rods
    for rod in &craft.rods {
        let (a, b) = (rod.node_a, rod.node_b);
        draw_line(
            craft.nodes[a].pos.x,
            craft.nodes[a].pos.y,
            craft.nodes[b].pos.x,
            craft.nodes[b].pos.y,
            2.0,
            match rod.rod_type {
                RodType::SOLID { .. } => DARKGRAY,
                RodType::ROPE { .. } => RED,
                RodType::SPRING {} => BLUE,
                RodType::PISTON { .. } => ORANGE,
            },
        );
    }

    // Draw nodes
    for node in &craft.nodes {
        draw_circle(
            node.pos.x,
            node.pos.y,
            6.0,
            match node.node_type {
                NodeType::Fixed => GREEN,
                NodeType::Joint => BLACK,
            },
        );
    }
}
