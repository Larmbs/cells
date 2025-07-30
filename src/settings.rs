use macroquad::prelude::{KeyCode, is_key_pressed};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum Action {
    /* Editor Controls */
    ClearPoints,

    PlaceNodes,
    PlaceRods,
    PlaceTriangles,

    Delete,

    MoveCamUp,
    MoveCameDown,
    MoveCamLeft,
    MoveCamRight,

    ZoomIn,
    ZoomOut,

    SwitchScene,
    NewCraft,
}

pub struct KeyBinds {
    action_to_key: HashMap<Action, KeyCode>,
}
impl KeyBinds {
    pub fn is_pressed(&self, action: Action) -> bool {
        if let Some(key) = self.action_to_key.get(&action) {
            is_key_pressed(*key)
        } else {
            false
        }
    }
}
impl Default for KeyBinds {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Action::ClearPoints, KeyCode::C);

        map.insert(Action::PlaceNodes, KeyCode::N);
        map.insert(Action::PlaceRods, KeyCode::R);
        map.insert(Action::PlaceTriangles, KeyCode::T);

        map.insert(Action::ZoomIn, KeyCode::Z);
        map.insert(Action::ZoomOut, KeyCode::X);

        map.insert(Action::Delete, KeyCode::Backspace);

        map.insert(Action::SwitchScene, KeyCode::Space);

        map.insert(Action::NewCraft, KeyCode::S);

        Self { action_to_key: map }
    }
}
