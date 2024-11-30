use crate::cell_colors::CellColors;

#[derive(Debug, PartialEq, Clone)]
pub enum Players {
    Player1,
    Player2,
}

impl Players {
    pub fn color(&self) -> CellColors {
        match self {
            Players::Player1 => CellColors::Blue,
            Players::Player2 => CellColors::Green,
        }
    }
}
