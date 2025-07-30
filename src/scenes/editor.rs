use super::Scene;
use crate::craft::editor::CraftManager;
use crate::craft::{Craft, components::*, draw_craft};
use crate::{
    AppMessage,
    settings::{Action, KeyBinds},
};

use macroquad::prelude::*;

const THRESHOLD: f32 = 20.0;

const SELECT_COLOR: Color = Color::from_hex(0x1ffdff);
// struct Drag {
//     start: Vec2,
//     drag_offset: Vec2,
// }
#[derive(Debug, Clone)]
enum Selected {
    New(Vec2),
    Node(usize),
    Rod(usize),
    Selected(usize),
}

pub struct Editor {
    manager: CraftManager,
    selected_points: Vec<Selected>,
    drag_start: Option<Vec2>,
    drag_current: Option<Vec2>,
}

impl Scene for Editor {
    fn update(&mut self, key_binds: &KeyBinds) -> AppMessage {
        let mouse = mouse_position().into();
        if key_binds.is_pressed(Action::SwitchScene) {
            return AppMessage::OpenSimulation(self.manager.c.clone());
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.selected_points.push(
                self.selected(mouse, THRESHOLD)
                    .unwrap_or((Selected::New(mouse), 0.0))
                    .0,
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_start = Some(mouse);
            self.drag_current = Some(mouse);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(_) = self.drag_start {
                self.drag_current = Some(mouse);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let (Some(start), Some(end)) = (self.drag_start, self.drag_current) {
                self.select_within_box(start, end);
            }
            self.drag_start = None;
            self.drag_current = None;
        }

        if key_binds.is_pressed(Action::NewCraft) {
            self.manager.c = Craft::new();
        }

        if key_binds.is_pressed(Action::ClearPoints) {
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceNodes) && self.selected_points.len() >= 1 {
            for point in &self.selected_points {
                let pos = match point {
                    Selected::New(pos) => *pos,
                    Selected::Node(id) => self.manager.c.nodes[*id].pos,
                    Selected::Rod(id) => self.manager.rod_midpoint(*id),
                    Selected::Selected(_) => continue,
                };
                self.manager.add_node(pos);
            }
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::PlaceRods) && self.selected_points.len() >= 2 {
            for i in 0..self.selected_points.len() - 1 {
                let node_a = self.ensure_node(i);
                let node_b = self.ensure_node(i + 1);

                self.manager.add_rod(node_a, node_b, RodType::default());
            }
            self.selected_points.clear();
        }

        if key_binds.is_pressed(Action::Delete) && !self.selected_points.is_empty() {
            let mut node_ids = Vec::new();
            let mut rod_ids = Vec::new();

            for sel in &self.selected_points {
                match sel {
                    Selected::Node(id) => node_ids.push(*id),
                    Selected::Rod(id) => rod_ids.push(*id),
                    Selected::New(_) | Selected::Selected(_) => {} // Ignore
                }
            }

            self.manager.remove_rods(&rod_ids);
            self.manager.remove_nodes(&node_ids);

            self.selected_points.clear();
        }
        AppMessage::None
    }

    fn draw(&self) {
        clear_background(WHITE);
        draw_craft(&self.manager.c);

        for sel in &self.selected_points {
            match sel {
                Selected::Rod(id) => {
                    let r = &self.manager.c.rods[*id];
                    let p1 = self.manager.c.nodes[r.node_a].pos;
                    let p2 = self.manager.c.nodes[r.node_b].pos;

                    draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, SELECT_COLOR);
                }
                _ => {
                    let pos = self.resolve_selected_point(&sel);
                    draw_circle(pos.x, pos.y, 6.0, SELECT_COLOR);
                }
            }
        }
        if let (Some(start), Some(end)) = (self.drag_start, self.drag_current) {
            let top_left = start.min(end);
            let size = (start - end).abs();
            draw_rectangle_lines(top_left.x, top_left.y, size.x, size.y, 1.0, SELECT_COLOR);
            draw_rectangle(
                top_left.x,
                top_left.y,
                size.x,
                size.y,
                SELECT_COLOR.with_alpha(0.7),
            );
        }
    }
}
impl Editor {
    pub fn new() -> Self {
        Self {
            manager: CraftManager { c: Craft::new() },
            selected_points: Vec::new(),
            drag_start: None,
            drag_current: None,
        }
    }

    pub fn edit_craft(craft: Craft) -> Self {
        Self {
            manager: CraftManager { c: craft },
            selected_points: Vec::new(),
            drag_start: None,
            drag_current: None,
        }
    }
}

/// Helper functions for selecting parts
impl Editor {
    fn resolve_selected_point(&self, selected: &Selected) -> Vec2 {
        match selected {
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
    fn selected_node(&self, pos: Vec2, threshold: f32) -> Option<(Selected, f32)> {
        let i = self.manager.select_nearest_node(pos)?;
        let dist = self.manager.c.nodes[i].pos.distance_squared(pos);
        if dist < threshold * threshold {
            return Some((Selected::Node(i), dist));
        }
        None
    }
    fn selected_rod(&self, pos: Vec2, threshold: f32) -> Option<(Selected, f32)> {
        let i = self.manager.select_nearest_rod(pos)?;
        let dist = self.manager.rod_midpoint(i).distance_squared(pos);
        if dist < threshold * threshold {
            return Some((Selected::Rod(i), dist));
        }
        None
    }
    fn selected_select(&self, pos: Vec2, threshold: f32) -> Option<(Selected, f32)> {
        self.selected_points
            .iter()
            .enumerate()
            .filter_map(|(i, sel)| {
                let dist = self.resolve_selected_point(sel).distance_squared(pos);
                if dist < threshold * threshold {
                    Some((Selected::Selected(i), dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }
    fn selected(&self, pos: Vec2, threshold: f32) -> Option<(Selected, f32)> {
        let mut candidates: Vec<(Selected, f32)> = vec![];

        if let Some(n) = self.selected_node(pos, threshold) {
            candidates.push(n);
        }
        if let Some(n) = self.selected_rod(pos, threshold) {
            candidates.push(n);
        }
        if let Some(n) = self.selected_select(pos, threshold) {
            candidates.push(n);
        }

        // Return the closest one
        candidates.into_iter().min_by(|a, b| a.1.total_cmp(&b.1))
    }
    /// Converts any selected point into a concrete node (by creating one if needed)
    fn ensure_node(&mut self, index: usize) -> usize {
        match self.selected_points[index] {
            Selected::New(pos) => {
                let id = self.manager.add_node(pos);
                self.selected_points[index] = Selected::Node(id);
                id
            }
            Selected::Node(id) => id,
            Selected::Rod(id) => {
                let pos = self.manager.rod_midpoint(id);
                let node_id = self.manager.add_node(pos);
                self.selected_points[index] = Selected::Node(node_id);
                node_id
            }
            Selected::Selected(id) => self.ensure_node(id),
        }
    }
    fn select_within_box(&mut self, start: Vec2, end: Vec2) {
        let min = start.min(end); // Top-left of the box
        let max = start.max(end); // Bottom-right

        //self.selected_points.clear(); // Optional: or merge with existing selection

        // Select nodes inside the box
        for (i, node) in self.manager.c.nodes.iter().enumerate() {
            if node.pos.x >= min.x
                && node.pos.x <= max.x
                && node.pos.y >= min.y
                && node.pos.y <= max.y
            {
                self.selected_points.push(Selected::Node(i));
            }
        }

        // Select rods whose *midpoint* is inside the box
        for (i, _) in self.manager.c.rods.iter().enumerate() {
            let mid = self.manager.rod_midpoint(i);
            if mid.x >= min.x && mid.x <= max.x && mid.y >= min.y && mid.y <= max.y {
                self.selected_points.push(Selected::Rod(i));
            }
        }
    }
}
