//! Vehicle Toolkit
//!
//! This modules defines a Craft editor
//! The craft editor allows for easy editable of a craft, providing a wide range of functions to
//! carry out edits on a craft
//!
//! A craft is a raw structure and has no checks on its own state

use super::{
    Craft,
    components::{Node, NodeType, Rod, RodType},
};
use macroquad::prelude::Vec2;
use std::collections::HashSet;

pub struct CraftManager {
    pub c: Craft,
}
impl CraftManager {
    const THRESHOLD: f32 = 50.0;

    /* Selecting craft components */
    pub fn select_nearest_node(&self, pos: Vec2) -> Option<usize> {
        self.c
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| {
                let dist = node.pos.distance_squared(pos);
                if dist < Self::THRESHOLD {
                    Some((i, dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }
    pub fn select_nearest_rod(&self, pos: Vec2) -> Option<usize> {
        self.c
            .rods
            .iter()
            .enumerate()
            .filter_map(|(i, rod)| {
                let a = self.c.nodes[rod.node_a].pos;
                let b = self.c.nodes[rod.node_b].pos;
                let midpoint = (a + b) * 0.5;
                let dist = midpoint.distance_squared(pos);
                if dist < Self::THRESHOLD * Self::THRESHOLD {
                    Some((i, dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| {
                println!("{:?}", a.1.total_cmp(&b.1));
                a.1.total_cmp(&b.1)
            })
            .map(|(i, _)| i)
    }

    /* Removing duplicates */
    pub fn remove_duplicate_nodes(&mut self) {
        let mut unique = vec![];
        let mut mapping = vec![None; self.c.nodes.len()];

        for (i, node) in self.c.nodes.iter().enumerate() {
            if let Some((idx, _)) = unique
                .iter()
                .enumerate()
                .find(|(_, n): &(_, &Node)| (n.pos - node.pos).length() < Self::THRESHOLD)
            {
                mapping[i] = Some(idx);
            } else {
                mapping[i] = Some(unique.len());
                unique.push(node.clone());
            }
        }

        self.c.nodes = unique;

        for rod in &mut self.c.rods {
            rod.node_a = mapping[rod.node_a].unwrap();
            rod.node_b = mapping[rod.node_b].unwrap();
        }
    }
    pub fn remove_duplicate_rods(&mut self) {
        let mut seen = HashSet::new();
        self.c
            .rods
            .retain(|rod| seen.insert([rod.node_a.min(rod.node_b), rod.node_a.max(rod.node_b)]));
    }

    /* Midpoint calculations */
    pub fn midpoint(&self, node_ids: &[usize]) -> Vec2 {
        node_ids
            .iter()
            .map(|&id| self.c.nodes[id].pos)
            .sum::<Vec2>()
            / node_ids.len() as f32
    }
    pub fn rod_midpoint(&self, rod_id: usize) -> Vec2 {
        self.midpoint(&[self.c.rods[rod_id].node_a, self.c.rods[rod_id].node_b])
    }

    /* Adding components to craft */
    pub fn add_node(&mut self, pos: Vec2, node_type: NodeType) -> usize {
        self.c.nodes.push(Node {
            pos,
            prev_pos: pos,
            node_type,
        });
        self.c.nodes.len() - 1
    }
    pub fn add_rod(&mut self, node1: usize, node2: usize, rod_type: RodType) -> usize {
        self.c.rods.push(Rod {
            node_a: node1,
            node_b: node2,
            length: (self.c.nodes[node1].pos - self.c.nodes[node2].pos).length(),
            rod_type,
        });
        self.c.rods.len() - 1
    }

    /* Removing components from craft */
    pub fn remove_node(&mut self, node_id: usize) -> Option<()> {
        if node_id >= self.c.nodes.len() {
            return None;
        }

        // Remove all rods and triangles connected to this node
        self.c
            .rods
            .retain(|rod| rod.node_a != node_id && rod.node_b != node_id);

        self.c.nodes.remove(node_id);

        // Fix indices in rods and triangles
        for rod in &mut self.c.rods {
            if rod.node_a > node_id {
                rod.node_a -= 1;
            }
            if rod.node_b > node_id {
                rod.node_b -= 1;
            }
        }

        Some(())
    }
    pub fn remove_rod(&mut self, rod_id: usize) -> Option<()> {
        if rod_id <= self.c.rods.len() {
            self.c.rods[rod_id] = self.c.rods[self.c.rods.len() - 1].clone();
            self.c.rods.remove(rod_id);
            Some(())
        } else {
            None
        }
    }

    /* Multi delete */
    pub fn remove_nodes(&mut self, node_ids: &[usize]) {
        let mut node_ids = node_ids.to_vec();
        node_ids.sort_unstable();

        let mut i = 0;
        while i < node_ids.len() {
            let id = node_ids[i];
            let last = self.c.nodes.len() - 1;

            if id >= self.c.nodes.len() {
                i += 1;
                continue;
            }

            // Remove rods connected to this node
            self.c
                .rods
                .retain(|rod| rod.node_a != id && rod.node_b != id);

            if id != last {
                // Swap node
                self.c.nodes.swap(id, last);

                // Update rods to point to new index if needed
                for rod in &mut self.c.rods {
                    if rod.node_a == last {
                        rod.node_a = id;
                    }
                    if rod.node_b == last {
                        rod.node_b = id;
                    }
                }

                // Update any upcoming deletions for the last index
                if let Some(pos) = node_ids.iter().position(|&x| x == last) {
                    node_ids[pos] = id;
                }
            }

            self.c.nodes.pop();
            i += 1;
        }
    }
    pub fn remove_rods(&mut self, rod_ids: &[usize]) {
        let mut rod_ids = rod_ids.to_vec();
        // Sort ascending for consistent processing
        rod_ids.sort_unstable();

        let mut i = 0;
        while i < rod_ids.len() {
            let id = rod_ids[i];
            let last = self.c.rods.len() - 1;

            if id >= self.c.rods.len() {
                i += 1;
                continue;
            }

            if id != last {
                // Find and update swapped index in our deletion list
                if let Some(pos) = rod_ids.iter().position(|&x| x == last) {
                    rod_ids[pos] = id;
                }
                self.c.rods.swap(id, last);
            }

            self.c.rods.pop();
            i += 1;
        }
    }

    /* Chain placement */
}
