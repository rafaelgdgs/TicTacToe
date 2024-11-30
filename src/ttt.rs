use crate::cell::Cell;
use crate::cell_played::CellPlayed;
use crate::players::Players;
use crate::pos_to_string;
use crate::ttt_color_to_macroquad_color;

#[derive(Debug, PartialEq)]
pub struct Ttt {
    pub board: Vec<Cell>,
    pub player: Players,
}

impl Ttt {
    pub fn new() -> Self {
        Self {
            board: vec![
                Cell::new(100f32, 100f32),
                Cell::new(200f32, 100f32),
                Cell::new(300f32, 100f32),
                Cell::new(100f32, 200f32),
                Cell::new(200f32, 200f32),
                Cell::new(300f32, 200f32),
                Cell::new(100f32, 300f32),
                Cell::new(200f32, 300f32),
                Cell::new(300f32, 300f32),
            ],
            player: Players::Player1,
        }
    }

    pub fn play(&mut self, position: usize) -> bool {
        if position > 8 {
            return false;
        }
        if self.board[position].is_played != CellPlayed::No {
            return false;
        }
        self.board[position].is_played = CellPlayed::Yes(self.player.to_owned());
        self.board[position].color =
            ttt_color_to_macroquad_color(self.board[position].is_played.color());
        self.player = match self.player {
            Players::Player1 => Players::Player2,
            Players::Player2 => Players::Player1,
        };
        true
    }

    pub fn find_cell(&mut self, cell: &Cell) -> Option<&mut Cell> {
        let mut into_iter = self.board.iter_mut();
        into_iter.find(|x| *x == cell)
    }

    pub fn play_cell(&mut self, cell: Cell) -> bool {
        let player = self.player.to_owned();
        let color = player.color();
        let bcell = self.find_cell(&cell);
        match bcell {
            None => false,
            Some(x) => {
                if x.is_played != CellPlayed::No {
                    return true;
                }
                x.is_played = CellPlayed::Yes(player);
                x.color = ttt_color_to_macroquad_color(color);
                self.player = match self.player {
                    Players::Player1 => Players::Player2,
                    Players::Player2 => Players::Player1,
                };
                true
            }
        }
    }

    pub fn pos(&self, position: usize) -> &CellPlayed {
        if position > 8 {
            return &CellPlayed::No;
        }
        &self.board[position].is_played
    }

    pub fn show(&self) {
        println!("==========");
        println!(
            "{}  {}  {}",
            pos_to_string(self, 0),
            pos_to_string(self, 1),
            pos_to_string(self, 2)
        );
        println!(
            "{}  {}  {}",
            pos_to_string(self, 3),
            pos_to_string(self, 4),
            pos_to_string(self, 5)
        );
        println!(
            "{}  {}  {}",
            pos_to_string(self, 6),
            pos_to_string(self, 7),
            pos_to_string(self, 8)
        );
        println!("==========");
    }

    pub fn get_player(&self) -> &Players {
        &self.player
    }
}
