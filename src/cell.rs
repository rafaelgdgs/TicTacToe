use crate::cell_played::CellPlayed;
use crate::ttt_color_to_macroquad_color;

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub is_played: CellPlayed,
    pub color: macroquad::color::Color,
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

impl Cell {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            is_played: CellPlayed::No,
            color: ttt_color_to_macroquad_color(CellPlayed::No.color()),
            x,
            y,
            size: 100f32,
        }
    }
}
