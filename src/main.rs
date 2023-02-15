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

        (Strategy::Scissors, Strategy::Paper) => (1, -1),
        (Strategy::Paper, Strategy::Scissors) => (-1, 1),

        (Strategy::Paper, Strategy::Rock) => (2, -2),
        (Strategy::Rock, Strategy::Paper) => (-2, 2),
        // // According to https://youtu.be/qOLXyFchZfY
        // // The result is rock 1/4, paper 1/4, scissors 1/2

        // (Strategy::Paper, Strategy::Rock) => (1, -1),
        // (Strategy::Rock, Strategy::Paper) => (-1, 1),

        (Strategy::Rock, Strategy::Scissors) => (1, -1),
        (Strategy::Scissors, Strategy::Rock) => (-1, 1),
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct Player {
    rock: u32,
    paper: u32,
    scissors: u32,
    score: i32,
    evolve_pt: u32, // 1/10000
}
impl Player {
    pub fn new() -> Self {
        Player {
            rock: 100000,
            paper: 100000,
            scissors: 100000,
            score: 0,
            evolve_pt: 1000,
        }
    }

    pub fn init_rock(mut self, rock: u32) -> Self {
        self.rock = rock;
        self
    }

    pub fn init_paper(mut self, paper: u32) -> Self {
        self.paper = paper;
        self
    }

    pub fn init_scissors(mut self, scissors: u32) -> Self {
        self.scissors = scissors;
        self
    }

    pub fn init_evolve(mut self, evolve_pt: u32) -> Self {
        self.evolve_pt = evolve_pt;
        self
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

    pub fn evolve(&mut self, rng: &mut ThreadRng) {
        if self.evolve_pt == 0 {
            return;
        }
        // Mutation
        // Why plus only?
        // The actual number does not affect the chance.
        // Only the relative ratio counts.

        // self.rock += rng.gen_range(0..1000);
        // self.paper += rng.gen_range(0..1000);
        // self.scissors += rng.gen_range(0..1000);

        self.rock += (self.rock * rng.gen_range(0..self.evolve_pt)) / 10000;
        self.paper += (self.paper * rng.gen_range(0..self.evolve_pt)) / 10000;
        self.scissors += (self.scissors * rng.gen_range(0..self.evolve_pt)) / 10000;

        // Cap the max at u32::MAX>>11
        if self.rock > u32::MAX>>11 || 
           self.paper > u32::MAX>>11 || 
           self.scissors > u32::MAX>>11 {
            self.rock = self.rock>>1;
            self.paper = self.paper>>1;
            self.scissors = self.scissors>>1;
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

    pub fn give_birth(&mut self, rng: &mut ThreadRng) -> Self {
        let mut player_offspring = self.clone();

        // This version is good. Make it more stable
        for _ in 0..5 {
            if self.evolve_pt > 1 {
                self.evolve_pt -= 1;
            }
        }

        if player_offspring.evolve_pt > 0 {
            player_offspring.evolve(rng);
        }
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

#[test]
fn test_purge_players() {
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new()
        .init_rock(100000)
        .init_paper(1)
        .init_scissors(1)
        .init_evolve(0)
    );
    players.push(Player::new()
        .init_rock(1)
        .init_paper(200000)
        .init_scissors(1)
        .init_evolve(0)
    );


    // Score < 0 removed, score > 0 remains.
    players[0].score += -1;
    players[1].score += 1;

    assert!(players.len() == 2);

    purge_players(&mut players);

    assert!(players.len() == 1, "Expect 1 player, see {}, players: {:?}", players.len(), players);


    // Score == 0 remains, do not remove.
    players.push(Player::new()
        .init_rock(1)
        .init_paper(1)
        .init_scissors(300000)
        .init_evolve(0)
    );

    purge_players(&mut players);

    assert!(players.len() == 2, "Expect 2 players, see {}, players: {:?}", players.len(), players);
}


fn level_playground(players: &mut Vec<Player>) {
    for player in &mut *players {
        player.score = 0;
    }
}

// TODO: Where should I put the rng? It is everywhere now.
fn reproduce_players(players: &mut Vec<Player>, rng: &mut ThreadRng) {
    let mut new_players: Vec<Player> = Vec::new();
    new_players.reserve(players.len() * 2);

    for player in &mut *players {
        if player.score > 0 {
            new_players.push(player.give_birth(rng));
        }
    }

    for _ in 0..players.len() {
        // See how the "food" affects the result
        players.push(Player::new()
            .init_rock(100000)
            .init_paper(1)
            .init_scissors(1)
            .init_evolve(0)
        );

        // players.push(Player::new()
        //     .init_rock(1)
        //     .init_paper(100000)
        //     .init_scissors(1)
        //     .init_evolve(0)
        // );
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

    // players.push(Player::new()
    //     .init_rock(100000)
    //     .init_paper(1000)
    //     .init_scissors(1000)
    // );

    // players.push(Player::new()
    //     .init_rock(1000)
    //     .init_paper(100000)
    //     .init_scissors(1000)
    // );

    // players.push(Player::new()
    //     .init_rock(1000)
    //     .init_paper(1000)
    //     .init_scissors(100000)
    // );

    for _ in 0..10 {
        players.push(Player::new()
            .init_rock(189541)
            .init_paper(189541)
            .init_scissors(189541*2)
            .init_evolve(0)
        );
        players.push(Player::new()
            .init_rock(100000)
            .init_paper(1)
            .init_scissors(1)
            .init_evolve(0)
        );
    }
    

    players.push(Player::new()
        .init_rock(100000)
        .init_paper(100000)
        .init_scissors(200000)
        .init_evolve(1000)
    );

    players.push(Player::new()
        .init_rock(100000)
        .init_paper(100000)
        .init_scissors(200000)
        .init_evolve(1000)
    );

    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));
    // players.push(Player::new().rand_init(&mut rng));

    // players.push(Player::new());
    // players.push(players[0].give_birth(&mut rng));

    dump(&players);

    let mut n_no_change = 0;

    // for round in 0..2000 {
    for round in 0..200 {
        println!("round: {round}");
        // Play a few rounds
        for _ in 0..17 {
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

        if round % 7 == 3 {
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

// TODO: Add tests
// TODO: Make it multithread, for learning Rust, and the speed.