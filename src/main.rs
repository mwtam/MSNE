use rand::{thread_rng, Rng, rngs::ThreadRng};

#[derive(Debug)]
enum Strategy {
    Rock,
    Paper,
    Scissors,
}

fn judge(player1_strategy: Strategy, player2_strategy: Strategy) -> (i32, i32) {
    match (player1_strategy, player2_strategy) {
        (Strategy::Rock, Strategy::Rock) => (0, 0),
        (Strategy::Paper, Strategy::Paper) => (0, 0),
        (Strategy::Scissors, Strategy::Scissors) => (0, 0),
        (Strategy::Rock, Strategy::Paper) => (-1, 1),
        (Strategy::Paper, Strategy::Scissors) => (-1, 1),
        (Strategy::Scissors, Strategy::Rock) => (-1, 1),
        (Strategy::Paper, Strategy::Rock) => (1, -1),
        (Strategy::Scissors, Strategy::Paper) => (1, -1),
        (Strategy::Rock, Strategy::Scissors) => (1, -1),
    }
}

#[derive(Debug)]
struct Player {
    rock: u32,
    paper: u32,
    scissors: u32,
    score: i32,
}
impl Player {
    pub fn new() -> Player {
        Player {
            rock: 100000,
            paper: 100000,
            scissors: 100000,
            score: 0,
        }
    }

    pub fn decide(&self, rng: &mut ThreadRng) -> Strategy {
        let mut choice: u32 = rng.gen_range(0..(self.rock + self.paper + self.scissors));

        if choice < self.rock {
            return Strategy::Rock;
        }

        choice -= self.rock;

        if choice < self.paper {
            return Strategy::Paper;
        }

        Strategy::Scissors
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", judge(Strategy::Rock, Strategy::Paper));
    println!("{:?}", judge(Strategy::Scissors, Strategy::Paper));

    let mut rng = thread_rng();
    rng.gen_range(0..10);

    let mut player_1 = Player::new();
    println!("Player 1: {:?}", player_1);
    println!("Player 1 decide: {:?}", player_1.decide(&mut rng));

    player_1.score += 1;
}
