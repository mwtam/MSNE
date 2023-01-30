
enum Strategy {
    Rock,
    Paper,
    Scissor,
}

fn judge(player1_strategy: Strategy, player2_strategy: Strategy) -> (i32, i32) {
    match (player1_strategy, player2_strategy) {
        (Strategy::Rock, Strategy::Rock) => (0, 0),
        (Strategy::Paper, Strategy::Paper) => (0, 0),
        (Strategy::Scissor, Strategy::Scissor) => (0, 0),
        (Strategy::Rock, Strategy::Paper) => (-1, 1),
        (Strategy::Paper, Strategy::Scissor) => (-1, 1),
        (Strategy::Scissor, Strategy::Rock) => (-1, 1),
        (Strategy::Paper, Strategy::Rock) => (1, -1),
        (Strategy::Scissor, Strategy::Paper) => (1, -1),
        (Strategy::Rock, Strategy::Scissor) => (1, -1),
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", judge(Strategy::Rock, Strategy::Paper));
    println!("{:?}", judge(Strategy::Scissor, Strategy::Paper));
}
