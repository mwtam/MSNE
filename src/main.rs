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

#[derive(Clone)]
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

    pub fn rand_init(mut self, rng: &mut ThreadRng) -> Player {
        self.rock = rng.gen_range(1..(u32::MAX>>12));
        self.paper = rng.gen_range(1..(u32::MAX>>12));
        self.scissors = rng.gen_range(1..(u32::MAX>>12));

        self
    }

    pub fn evolute(&mut self, rng: &mut ThreadRng) {
        // Mutation
        // Why plus only?
        // The actual number does not affect the chance.
        // Only the relative ratio counts.
        self.rock += rng.gen_range(0..1000);
        self.paper += rng.gen_range(0..1000);
        self.scissors += rng.gen_range(0..1000);

        // Cap the max at u32::MAX>>11
        if self.rock > u32::MAX>>11 || 
           self.paper > u32::MAX>>11 || 
           self.scissors > u32::MAX>>11 {
            self.rock = self.rock>>1;
            self.paper = self.rock>>1;
            self.scissors = self.rock>>1;
        }

        // Cap the min at 0
        if self.rock == 0 {
            self.rock = 1;
        }
        if self.paper == 0 {
            self.paper = 1;
        }
        if self.scissors == 0 {
            self.scissors = 1;
        }
    }

    pub fn give_birth(&self, rng: &mut ThreadRng) -> Player {
        let mut player_offspring = self.clone();
        player_offspring.evolute(rng);
        player_offspring
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
    let mut rng = thread_rng();

    let mut players: Vec<Player> = vec![];
    players.push(Player::new().rand_init(&mut rng));
    players.push(Player::new().rand_init(&mut rng));
    players.push(Player::new().rand_init(&mut rng));
    players.push(Player::new());
    players.push(players[0].give_birth(&mut rng));

    // let mut player_offspring = players[0].clone();
    // player_offspring.evolute(&mut rng);
    // players.push(player_offspring);

    // Play a few rounds
    for _ in 0..100000 {
        for i in 0..players.len() {
            for j in (i+1)..players.len() {
                let game = (players[i].decide(&mut rng), players[j].decide(&mut rng));
                // println!("Game {i}-{j}: {:?}", game);
                let result = judge(game.0, game.1);

                players[i].score += result.0;
                players[j].score += result.1;
            }
        }
    }

    for player in players {
        println!("Player: {:?}", player);
    }
}

// It is expected playing rock, paper, scissors equally the equilibrium.
// What if I purposely provide "food" who always play rock.
// Then playing paper more is good because they can eat the food.
// However, if so, playing scissors may eat those play paper more.
// Then who's "food"?