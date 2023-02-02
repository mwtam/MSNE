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
    pub fn new() -> Self {
        Player {
            rock: 100000,
            paper: 100000,
            scissors: 100000,
            score: 0,
        }
    }

    pub fn rang_parameters(&mut self, rng: &mut ThreadRng) {
        self.rock = rng.gen_range(1..(u32::MAX>>12));
        self.paper = rng.gen_range(1..(u32::MAX>>12));
        self.scissors = rng.gen_range(1..(u32::MAX>>12));
    }

    pub fn rand_init(mut self, rng: &mut ThreadRng) -> Self {
        self.rang_parameters(rng);
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

    pub fn give_birth(&self, rng: &mut ThreadRng) -> Self {
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

fn play_all_pairs(players: &mut Vec<Player>, rng: &mut ThreadRng) {
    for i in 0..players.len() {
        for j in (i+1)..players.len() {
            let game = (players[i].decide(rng), players[j].decide(rng));
            // println!("Game {i}-{j}: {:?}", game);
            let result = judge(game.0, game.1);

            players[i].score += result.0;
            players[j].score += result.1;
        }
    }
}

fn purge_players(players: &mut Vec<Player>) {
    players.retain(|player| !player.score.is_negative());
}

fn level_playground(players: &mut Vec<Player>) {
    for player in &mut *players {
        player.score = 0;
    }
}

// TODO: Where should I put the rng? It is everywhere now.
fn reproduce_players(players: &mut Vec<Player>, rng: &mut ThreadRng) {
    let mut new_players: Vec<Player> = Vec::new();
    for player in &mut *players {
        if player.score > 0 {
            new_players.push(player.give_birth(rng));
        }
    }

    players.append(&mut new_players);
}


fn dump(players: &Vec<Player>) {
    for player in players {
        println!("Player: {:?}", player);
    }
}

fn main() {
    let mut rng = thread_rng();

    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new().rand_init(&mut rng));
    players.push(Player::new().rand_init(&mut rng));
    players.push(Player::new().rand_init(&mut rng));
    players.push(Player::new());
    players.push(players[0].give_birth(&mut rng));

    let mut n_no_change = 0;

    for round in 0..1000 {
        println!("round: {round}");
        // Play a few rounds
        for _ in 0..100 {
            play_all_pairs(&mut players, &mut rng);
        }

        // dump(&players);
        println!("population: {}", players.len());

        if players.len() < 800 {
            reproduce_players(&mut players, &mut rng);
            level_playground(&mut players);
            n_no_change = 0;
        }
        if players.len() > 1000 {
            let n_population = players.len();
            purge_players(&mut players);
            level_playground(&mut players);
            if n_population != players.len() {
                n_no_change = 0;
            }
        }
        if round % 7 == 0 {
            let n_population = players.len();
            purge_players(&mut players);
            level_playground(&mut players);
            if n_population != players.len() {
                n_no_change = 0;
            }
        }

        n_no_change += 1;
        if n_no_change > 10 {
            break;
        }
        // println!("--------");
        // dump(&players);

        // println!("========");
    }
    dump(&players);
}

// TODO: Make it multithread for learning Rust and faster.

// It is expected playing rock, paper, scissors equally the equilibrium.
// What if I purposely provide "food" who always play rock.
// Then playing paper more is good because they can eat the food.
// However, if so, playing scissors may eat those play paper more.
// Then who's "food"?