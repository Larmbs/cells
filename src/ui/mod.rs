//! UI System
//!
//! This module allows for the easy creation of ui for games with a clean style
#![allow(unused)]

use macroquad::prelude::Rect;

mod widgets;
use widgets::UiWidget;

mod style;

/// Defines the direction in which widgets are drawn
enum Layout {
    Row,
    Column,
}
/// A box with Ui elements
pub struct UiComponent {
    /// Rectangle displaying the UI
    rect: Rect,
    /// An array of widgets
    widgets: Vec<Box<dyn UiWidget>>,
    layout: Layout,
}
