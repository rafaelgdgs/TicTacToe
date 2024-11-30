mod cell;
mod cell_colors;
mod cell_played;
mod funcs;
mod players;
mod ttt;

use crate::cell::Cell;
use crate::cell_colors::CellColors;
use crate::cell_played::CellPlayed;
use crate::funcs::*;
use crate::players::Players;
use crate::ttt::Ttt;
//use macroquad::prelude::*;

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut board = Ttt::new();
    let stdin_play = false;

    loop {
        while !is_victory(&board) {
            if stdin_play {
                play_on_stdin(&mut board);
            } else {
                play_on_gui(&mut board).await;
            }
        }
        game_restart(&mut board).await;
    }
}
