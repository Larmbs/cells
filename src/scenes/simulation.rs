//! Vehicle Toolkit
//! Simulation Scene
use macroquad::prelude::{
    GREEN, Vec2, WHITE, clear_background, draw_line, get_frame_time, get_time, screen_width,
};

use super::Scene;
use crate::{
    AppMessage,
    settings::{Action, KeyBinds},
};

use crate::craft::*;
use crate::craft::{components::*, draw_craft};

const FLOOR: f32 = 600.0;

pub struct Simulation {
    gravity: Vec2,
    original_craft: Craft,
    craft: Craft,
}
impl Scene for Simulation {
    fn update(&mut self, key_binds: &KeyBinds) -> AppMessage {
        // Changes scene to Editor
        if key_binds.is_pressed(Action::SwitchScene) {
            return AppMessage::OpenEditor(Some(self.original_craft.clone()));
        }

        let dt = get_frame_time() as f32;

        // Verlet integration
        for node in &mut self.craft.nodes {
            if let NodeType::Fixed = node.node_type {
                continue;
            }
            let temp = node.pos;
            let velocity = node.pos - node.prev_pos;
            node.pos += velocity + self.gravity * dt * dt;
            node.prev_pos = temp;

            // Floor collision
            if node.pos.y > FLOOR {
                node.pos.y = FLOOR;
                let mut velocity = node.pos - node.prev_pos;
                velocity.y *= -0.3;
                node.prev_pos = node.pos - velocity;
            }
        }

        // Rod constraints
        for _ in 0..5 {
            for rod in self.craft.rods.clone().iter() {
                let (a, b) = (rod.node_a, rod.node_b);
                let (pa, pb) = (self.craft.nodes[a].pos, self.craft.nodes[b].pos);
                let delta = pb - pa;
                let dist = delta.length();
                if dist == 0.0 {
                    continue;
                }
                let dir = delta / dist;

                match rod.rod_type {
                    RodType::SOLID => {
                        let diff = dist - rod.length;
                        let correction = dir * (diff * 0.5);
                        self.move_nodes(a, b, correction);
                    }
                    RodType::ROPE => {
                        if dist > rod.length {
                            let diff = dist - rod.length;
                            let correction = dir * (diff * 0.5);
                            self.move_nodes(a, b, correction);
                        }
                    }
                    RodType::SPRING => {
                        // Simple spring: pull or push nodes toward rest length
                        let rest_length = 100.0;
                        let k = 0.2;
                        let force = dir * (dist - rest_length) * k;
                        self.move_nodes(a, b, force);
                    }
                    RodType::PISTON => {
                        // Dynamic length, could be user-controlled or animated
                        // For now, just placeholder behavior
                        let desired_length = 150.0 + 50.0 * (get_time() as f32).sin();
                        let diff = dist - desired_length;
                        let correction = dir * (diff * 0.5);
                        self.move_nodes(a, b, correction);
                    }
                }
            }
        }
        AppMessage::None
    }

    fn draw(&self) {
        clear_background(WHITE);
        draw_craft(&self.craft);
        draw_line(0.0, 600.0, screen_width(), 600.0, 2.0, GREEN);
    }
}
impl Simulation {
    pub fn new(craft: Craft) -> Self {
        Self {
            gravity: Vec2::new(0.0, 500.0),
            original_craft: craft.clone(),
            craft,
        }
    }

    fn move_nodes(&mut self, a: usize, b: usize, correction: Vec2) {
        if self.craft.nodes[a].node_type != NodeType::Fixed {
            self.craft.nodes[a].pos += correction;
        }
        if self.craft.nodes[b].node_type != NodeType::Fixed {
            self.craft.nodes[b].pos -= correction;
        }
    }
}
