use macroquad::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Players {
    Player1,
    Player2,
}

impl Players {
    fn color(&self) -> CellColors {
        match self {
            Players::Player1 => CellColors::Blue,
            Players::Player2 => CellColors::Green,
        }
    }
}

enum CellColors {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Clone)]
enum CellPlayed {
    Yes(Players),
    No,
}

impl CellPlayed {
    fn color(&self) -> CellColors {
        match self {
            CellPlayed::Yes(p) => p.color(),
            CellPlayed::No => CellColors::Red,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ttt {
    board: Vec<CellPlayed>,
    player: Players,
}

impl Ttt {
    fn new() -> Self {
        Self {
            board: vec![CellPlayed::No; 9],
            player: Players::Player1,
        }
    }

    fn play(&mut self, position: usize) -> bool {
        if position > 9 {
            return false;
        }
        if self.board[position] != CellPlayed::No {
            return false;
        }
        self.board[position] = CellPlayed::Yes(self.player.to_owned());
        self.player = match self.player {
            Players::Player1 => Players::Player2,
            Players::Player2 => Players::Player1,
        };
        true
    }

    fn pos(&self, position: usize) -> &CellPlayed {
        if position > 9 {
            return &CellPlayed::No;
        }
        &self.board[position]
    }

    fn show(&self) {
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

    fn get_player(&self) -> &Players {
        &self.player
    }
}

fn pos_to_string(board: &Ttt, pos: usize) -> String {
    match board.pos(pos) {
        CellPlayed::No => "0".to_string(),
        CellPlayed::Yes(x) => match x {
            Players::Player1 => "1".to_string(),
            Players::Player2 => "2".to_string(),
        },
    }
}

fn player_to_string(p: &Players) -> String {
    match p {
        Players::Player1 => "1".to_string(),
        Players::Player2 => "2".to_string(),
    }
}

fn winner_player_based_on_current_player(board: &Ttt) -> &Players {
    match board.get_player() {
        Players::Player1 => &Players::Player2,
        Players::Player2 => &Players::Player1,
    }
}

fn is_victory(board: &Ttt) -> bool {
    if board.pos(0) != &CellPlayed::No
        && board.pos(0) == board.pos(1)
        && board.pos(0) == (board.pos(2))
    {
        return true;
    }
    if board.pos(3) != &CellPlayed::No
        && board.pos(3) == board.pos(4)
        && board.pos(3) == board.pos(5)
    {
        return true;
    }
    if board.pos(6) != &CellPlayed::No
        && board.pos(6) == board.pos(7)
        && board.pos(6) == board.pos(8)
    {
        return true;
    }
    if board.pos(0) != &CellPlayed::No
        && board.pos(0) == board.pos(3)
        && board.pos(0) == board.pos(6)
    {
        return true;
    }
    if board.pos(1) != &CellPlayed::No
        && board.pos(1) == board.pos(4)
        && board.pos(1) == board.pos(7)
    {
        return true;
    }
    if board.pos(2) != &CellPlayed::No
        && board.pos(2) == board.pos(5)
        && board.pos(2) == board.pos(8)
    {
        return true;
    }
    if board.pos(0) != &CellPlayed::No
        && board.pos(0) == board.pos(4)
        && board.pos(0) == board.pos(8)
    {
        return true;
    }
    if board.pos(2) != &CellPlayed::No
        && board.pos(2) == board.pos(4)
        && board.pos(2) == board.pos(6)
    {
        return true;
    }
    false
}

// impl<'a, 'b> PartialEq<

// fn calculate_win(b: Ttt) -> Vec<Ttt> {}

fn draw_game(game: &Ttt) {
    let width = 100.0;
    let heigth = 100.0;
    clear_background(WHITE);
    let mut linha = 1f32;
    let mut coluna = 1f32;
    for item in &game.board {
        if coluna >= 4f32 {
            linha += 1f32;
            coluna = 1f32;
        }
        match item {
            CellPlayed::No => {
                draw_rectangle(
                    coluna * width + 30f32,
                    linha * heigth + 30f32,
                    width,
                    heigth,
                    RED,
                );
            }
            CellPlayed::Yes(p) => match p {
                Players::Player1 => {
                    draw_rectangle(
                        coluna * width + 30f32,
                        linha * heigth + 30f32,
                        width,
                        heigth,
                        BLUE,
                    );
                }
                Players::Player2 => {
                    draw_rectangle(
                        coluna * width + 30f32,
                        linha * heigth + 30f32,
                        width,
                        heigth,
                        GREEN,
                    );
                }
            },
        }
        coluna += 1f32;
    }
}

#[macroquad::main("TicTacToe")]
async fn main() {
    use std::io::{stdin, stdout, Write};
    let mut board = Ttt::new();
    // board.show();
    while !is_victory(&board) {
        print!("Next play: ");
        let mut s = String::new();
        let _ = stdout().flush();
        while s.is_empty() {
            stdin().read_line(&mut s).expect("Incorrect string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
        }
        if !board.play(s.parse::<usize>().unwrap() - 1) {
            println!("Problem playing it. Nothing happened");
        }
        draw_game(&board);
        // board.show();
        next_frame().await
    }
    println!(
        "Player {} won!",
        player_to_string(winner_player_based_on_current_player(&board))
    );
}
