#[derive(Debug, PartialEq, Clone)]
enum Players {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq, Clone)]
enum CellPlayer {
    Player(Players),
    NoOne,
}

#[derive(Debug, PartialEq)]
struct Ttt {
    board: Vec<CellPlayer>,
    player: Players,
}

impl Ttt {
    fn new() -> Self {
        Self {
            board: vec![CellPlayer::NoOne; 9],
            player: Players::Player1,
        }
    }

    fn play(&mut self, position: usize) -> bool {
        if position > 9 {
            return false;
        }
        if self.board[position] != CellPlayer::NoOne {
            return false;
        }
        self.board[position] = CellPlayer::Player(self.player.to_owned());
        self.player = match self.player {
            Players::Player1 => Players::Player2,
            Players::Player2 => Players::Player1,
        };
        true
    }

    fn pos(&self, position: usize) -> &CellPlayer {
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

    fn getPlayer(&self) -> &Players {
        &self.player
    }
}

fn pos_to_string(board: &Ttt, pos: usize) -> String {
    match board.pos(pos) {
        CellPlayer::NoOne => "0".to_string(),
        CellPlayer::Player(x) => match x {
            Players::Player1 => "1".to_string(),
            Players::Player2 => "2".to_string(),
        },
    }
}

fn winner_player_based_on_current_player(board: &Ttt) -> String {
    match board.getPlayer() {
        Players::Player1 => "2".to_string(),
        Players::Player2 => "1".to_string(),
    }
}

fn is_victory(board: &Ttt) -> bool {
    if board.pos(0) != &CellPlayer::NoOne
        && board.pos(0) == board.pos(1)
        && board.pos(0) == (board.pos(2))
    {
        return true;
    }
    if board.pos(3) != &CellPlayer::NoOne
        && board.pos(3) == board.pos(4)
        && board.pos(3) == board.pos(5)
    {
        return true;
    }
    if board.pos(6) != &CellPlayer::NoOne
        && board.pos(6) == board.pos(7)
        && board.pos(6) == board.pos(8)
    {
        return true;
    }
    if board.pos(0) != &CellPlayer::NoOne
        && board.pos(0) == board.pos(3)
        && board.pos(0) == board.pos(6)
    {
        return true;
    }
    if board.pos(1) != &CellPlayer::NoOne
        && board.pos(1) == board.pos(4)
        && board.pos(1) == board.pos(7)
    {
        return true;
    }
    if board.pos(2) != &CellPlayer::NoOne
        && board.pos(2) == board.pos(5)
        && board.pos(2) == board.pos(8)
    {
        return true;
    }
    if board.pos(0) != &CellPlayer::NoOne
        && board.pos(0) == board.pos(4)
        && board.pos(0) == board.pos(8)
    {
        return true;
    }
    if board.pos(2) != &CellPlayer::NoOne
        && board.pos(2) == board.pos(4)
        && board.pos(2) == board.pos(6)
    {
        return true;
    }
    false
}

// impl<'a, 'b> PartialEq<

// fn calculate_win(b: Ttt) -> Vec<Ttt> {}

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut board = Ttt::new();
    board.show();
    while !is_victory(&board) {
        print!("Next play: ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Incorrect string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        if !board.play(s.parse::<usize>().unwrap() - 1) {
            println!("Problem playing it. Nothing happened");
        }
        //println!("{:?}", board);
        board.show();
    }
    println!(
        "Player {} won!",
        winner_player_based_on_current_player(&board)
    );
    // println!("{:?}", board);
    // board.play(2);
    // board.play(0);
    // println!("{:?}", board);
    // println!("Hello.")
}
