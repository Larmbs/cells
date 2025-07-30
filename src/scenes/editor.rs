use super::Scene;
use crate::craft::editor::CraftManager;
use crate::craft::{Craft, components::*, draw_craft};
use crate::{
    AppMessage,
    settings::{Action, KeyBinds},
};

use macroquad::prelude::*;

const THRESHOLD: f32 = 20.0;

#[derive(Debug)]
enum Selected {
    New(Vec2),
    Node(usize),
    Rod(usize),
    Selected(usize),
}

pub struct Editor {
    manager: CraftManager,
    selected_points: Vec<Selected>,
}

impl Scene for Editor {
    fn update(&mut self, key_binds: &KeyBinds) -> AppMessage {
        if key_binds.is_pressed(Action::SwitchScene) {
            return AppMessage::OpenSimulation(self.manager.c.clone());
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse = mouse_position().into();
            self.selected_points.push(
                self.selected(mouse, THRESHOLD)
                    .unwrap_or(Selected::New(mouse)),
            );
        }

        if key_binds.is_pressed(Action::ClearPoints) {
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceNodes) {
            for point in &self.selected_points {
                let pos = match point {
                    Selected::New(pos) => *pos,
                    Selected::Node(id) => self.manager.c.nodes[*id].pos,
                    Selected::Rod(id) => self.manager.rod_midpoint(*id),
                    Selected::Selected(_) => continue,
                };
                self.manager.add_node(pos, NodeType::default());
            }
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceRods) {
            for i in 0..self.selected_points.len() - 1 {
                let node_a = self.ensure_node(i);
                let node_b = self.ensure_node(i + 1);

                self.manager.add_rod(node_a, node_b, RodType::default());
            }
            self.selected_points.clear();
        }

        AppMessage::None
    }

    fn draw(&self) {
        clear_background(WHITE);
        draw_craft(&self.manager.c);

        for point in &self.selected_points {
            let pos = match point {
                Selected::New(p) => *p,
                Selected::Node(id) => self.manager.c.nodes[*id].pos,
                Selected::Rod(id) => self.manager.rod_midpoint(*id),
                Selected::Selected(_) => continue,
            };
            draw_circle(pos.x, pos.y, 6.0, SKYBLUE);
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            manager: CraftManager { c: Craft::new() },
            selected_points: Vec::new(),
        }
    }

    pub fn edit_craft(craft: Craft) -> Self {
        Self {
            manager: CraftManager { c: craft },
            selected_points: Vec::new(),
        }
    }

    /// Converts any selected point into a concrete node (by creating one if needed)
    fn ensure_node(&mut self, index: usize) -> usize {
        match self.selected_points[index] {
            Selected::New(pos) => {
                let id = self.manager.add_node(pos, NodeType::default());
                self.selected_points[index] = Selected::Node(id);
                id
            }
            Selected::Node(id) => id,
            Selected::Rod(id) => {
                let pos = self.manager.rod_midpoint(id);
                let node_id = self.manager.add_node(pos, NodeType::default());
                self.selected_points[index] = Selected::Node(node_id);
                node_id
            }
            Selected::Selected(id) => self.ensure_node(id),
        }
    }

    fn resolve_selected_point(&self, sel: &Selected) -> Vec2 {
        match sel {
            Selected::New(pos) => *pos,
            Selected::Node(id) => self.manager.c.nodes[*id].pos,
            Selected::Rod(id) => self.manager.rod_midpoint(*id),
            Selected::Selected(index) => {
                if let Some(inner) = self.selected_points.get(*index) {
                    self.resolve_selected_point(inner)
                } else {
                    Vec2::ZERO // fallback for invalid reference
                }
            }
        }
    }

    fn selected(&self, pos: Vec2, threshold: f32) -> Option<Selected> {
        let mut candidates: Vec<(Selected, f32)> = vec![];

        // Check nearest node
        if let Some(i) = self.manager.select_nearest_node(pos) {
            let dist = self.manager.c.nodes[i].pos.distance_squared(pos);
            if dist < threshold * threshold {
                candidates.push((Selected::Node(i), dist));
            }
        }

        // Check nearest rod
        if let Some(i) = self.manager.select_nearest_rod(pos) {
            let dist = self.manager.rod_midpoint(i).distance_squared(pos);
            if dist < threshold * threshold {
                candidates.push((Selected::Rod(i), dist));
            }
        }

        // Check existing selected points
        for (i, sel) in self.selected_points.iter().enumerate() {
            let point_pos = self.resolve_selected_point(sel);
            let dist = point_pos.distance_squared(pos);
            if dist < threshold * threshold {
                candidates.push((Selected::Selected(i), dist));
            }
        }

        // Return the closest one
        candidates
            .into_iter()
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(sel, _)| sel)
    }
}
