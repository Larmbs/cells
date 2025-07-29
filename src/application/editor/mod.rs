use crate::application::{
    AppMessage, Scene,
    settings::{Action, KeyBinds},
};

use super::craft::components::*;
use super::craft::*;
use macroquad::window::clear_background;
use macroquad::{
    color::SKYBLUE,
    prelude::{
        Color, GREEN, KeyCode, MouseButton, Vec2, WHITE, draw_circle, is_key_pressed,
        is_mouse_button_pressed, mouse_position,
    },
};

const THRESHOLD: f32 = 50.0;

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
    selected_points: Vec<Vec2>,
    picked_color: Color,
}
impl Scene for Editor {
    fn update(&mut self, key_binds: &KeyBinds) -> AppMessage {
        // Changes scene to Simulate
        if key_binds.is_pressed(Action::SwitchScene) {
            return AppMessage::OpenSimulation(self.craft.clone());
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.selected_points.push(
                if let Some(id) = self.find_closest_node(mouse_position().into(), THRESHOLD) {
                    self.craft.nodes[id].pos.clone()
                } else {
                    mouse_position().into()
                },
            );
        }
        if key_binds.is_pressed(Action::ClearPoints) {
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceNodes) {
            for i in 0..self.selected_points.len() {
                if self
                    .find_closest_node(self.selected_points[i], THRESHOLD)
                    .is_none()
                {
                    self.add_node(self.selected_points[i], NodeType::default());
                }
            }
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceRods) {
            for i in 1..self.selected_points.len() {
                let node_a = if let Some(id) =
                    self.find_closest_node(self.selected_points[i - 1], THRESHOLD)
                {
                    id
                } else {
                    self.add_node(self.selected_points[i - 1], NodeType::default())
                };

                let node_b =
                    if let Some(id) = self.find_closest_node(self.selected_points[i], THRESHOLD) {
                        id
                    } else {
                        self.add_node(self.selected_points[i], NodeType::default())
                    };

                self.add_rod(
                    node_a,
                    node_b,
                    self.craft.node_distance(node_a, node_b),
                    RodType::default(),
                );
            }
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceTriangles) {
            //let mut last = None;
            self.selected_points.clear();
        }

        super::AppMessage::None
    }

    fn draw(&self) {
        clear_background(WHITE);
        self.craft.draw();
        for point in &self.selected_points {
            draw_circle(point.x, point.y, 6.0, SKYBLUE);
        }
    }
}
impl Editor {
    pub fn new() -> Self {
        Self {
            mode: EditMode::EditNodes,
            craft: Craft::new(),
            selected_points: Vec::new(),
            picked_color: GREEN,
        }
    }
    pub fn edit_craft(craft: Craft) -> Self {
        Self {
            mode: EditMode::EditNodes,
            craft,
            selected_points: Vec::new(),
            picked_color: GREEN,
        }
    }

    fn add_node(&mut self, pos: Vec2, node_type: NodeType) -> usize {
        self.craft.nodes.push(Node {
            pos,
            prev_pos: pos.clone(),
            node_type,
        });
        self.craft.nodes.len() - 1
    }

    fn add_rod(&mut self, node_a: usize, node_b: usize, length: f32, rod_type: RodType) {
        self.craft.rods.push(Rod {
            node_a,
            node_b,
            length,
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
