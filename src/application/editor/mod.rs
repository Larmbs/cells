use crate::application::{AppMessage, Scene};

use super::craft::components::*;
use super::craft::*;
use macroquad::prelude::{
    Color, GREEN, KeyCode, MouseButton, Vec2, WHITE, is_key_pressed, is_mouse_button_pressed,
    mouse_position,
};
use macroquad::window::clear_background;

enum EditMode {
    EditNodes,
    EditRods,
    EditTriangles,
}
impl EditMode {
    fn swap(&self) -> Self {
        match self {
            EditMode::EditNodes => EditMode::EditRods,
            EditMode::EditRods => EditMode::EditTriangles,
            EditMode::EditTriangles => EditMode::EditNodes,
        }
    }
}
pub struct Editor {
    mode: EditMode,
    craft: Craft,
    selected_nodes: Vec<usize>,
    picked_color: Color,
}
impl Scene for Editor {
    fn update(&mut self) -> AppMessage {
        // Changes scene to Simulate
        if is_key_pressed(KeyCode::Space) {
            return AppMessage::OpenSimulation(self.craft.clone());
        }

        // Swaps build mode
        if is_key_pressed(KeyCode::M) {
            self.mode = self.mode.swap();
            self.selected_nodes = Vec::new();
        }

        /// Place
        if is_mouse_button_pressed(MouseButton::Left) {
            match self.mode {
                EditMode::EditNodes => self.add_node(mouse_position().into(), NodeType::Joint),
                EditMode::EditRods => {
                    let selected_node = self.find_closest_node(mouse_position().into(), 25.0);
                    if let Some(node_id) = selected_node {
                        self.selected_nodes.push(node_id);
                        if self.selected_nodes.len() == 2 {
                            self.add_rod(
                                self.selected_nodes[0],
                                self.selected_nodes[1],
                                RodType::SOLID {
                                    length: (self.craft.nodes[self.selected_nodes[0]].pos
                                        - self.craft.nodes[self.selected_nodes[1]].pos)
                                        .length(),
                                    strength: 0.0,
                                },
                            );
                            self.selected_nodes.clear();
                        }
                    } else {
                        self.selected_nodes.clear();
                    }
                }
                EditMode::EditTriangles => {
                    let selected_node = self.find_closest_node(mouse_position().into(), 25.0);
                    if let Some(node_id) = selected_node {
                        self.selected_nodes.push(node_id);
                        if self.selected_nodes.len() == 3 {
                            self.add_triangle(
                                self.selected_nodes[0],
                                self.selected_nodes[1],
                                self.selected_nodes[2],
                            );
                            self.selected_nodes.clear();
                        }
                    } else {
                        self.selected_nodes.clear();
                    }
                }
            }
        }

        // /// Delete
        // if is_mouse_button_pressed(MouseButton::Right) {
        //     if let Some(idx) = find_closest_node(&sim.nodes, mouse, 15.0) {
        //         if let Some(prev) = selected_node {
        //             if prev != idx {
        //                 sim.add_rod(
        //                     prev,
        //                     idx,
        //                     RodType::SOLID {
        //                         length: (sim.nodes[prev].pos - sim.nodes[idx].pos).length(),
        //                     },
        //                 );
        //             }
        //             selected_node = None;
        //         } else {
        //             selected_node = Some(idx);
        //         }
        //     }
        // }

        super::AppMessage::None
    }

    fn draw(&self) {
        clear_background(WHITE);
        self.craft.draw(&self.selected_nodes);
    }
}
impl Editor {
    pub fn new() -> Self {
        Self {
            mode: EditMode::EditNodes,
            craft: Craft::new(),
            selected_nodes: Vec::new(),
            picked_color: GREEN,
        }
    }
    pub fn edit_craft(craft: Craft) -> Self {
        Self {
            mode: EditMode::EditNodes,
            craft,
            selected_nodes: Vec::new(),
            picked_color: GREEN,
        }
    }

    fn add_node(&mut self, pos: Vec2, node_type: NodeType) {
        self.craft.nodes.push(Node {
            pos,
            prev_pos: pos.clone(),
            node_type,
        })
    }

    fn add_rod(&mut self, node_a: usize, node_b: usize, rod_type: RodType) {
        self.craft.rods.push(Rod {
            node_a,
            node_b,
            rod_type,
        });
    }

    fn add_triangle(&mut self, node_a: usize, node_b: usize, node_c: usize) {
        self.craft.triangles.push(Triangle {
            node_a,
            node_b,
            node_c,
            color: self.picked_color,
        });
    }

    fn remove_node(&mut self, index: usize) {
        todo!()
    }

    fn remove_rod(&mut self, index: usize) {
        todo!()
    }

    fn remove_triangle(&mut self, index: usize) {
        todo!()
    }

    fn find_closest_node(&self, pos: Vec2, threshold: f32) -> Option<usize> {
        self.craft
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                let d = (n.pos - pos).length();
                if d < threshold { Some((i, d)) } else { None }
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }

    pub fn close(self) -> Craft {
        self.craft
    }
}
