use crate::cell_colors::CellColors;
use crate::players::Players;

#[derive(Debug, PartialEq, Clone)]
pub enum CellPlayed {
    Yes(Players),
    No,
}

impl CellPlayed {
    pub fn color(&self) -> CellColors {
        match self {
            CellPlayed::Yes(p) => p.color(),
            CellPlayed::No => CellColors::Purple,
        }
    }
}
