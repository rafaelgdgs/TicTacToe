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
}

// impl<'a, 'b> PartialEq<

// fn calculate_win(b: Ttt) -> Vec<Ttt> {}

fn main() {
    let mut a = Ttt::new();
    println!("{:?}", a);
    a.play(2);
    a.play(0);
    println!("{:?}", a);
    println!("Hello.")
}
