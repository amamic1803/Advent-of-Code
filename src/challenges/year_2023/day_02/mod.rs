use crate::shared::structures::Day;

pub fn day_02() -> Day {
    Day::new(
        2,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

const CUBE_AMOUNTS: [u32; 3] = [12, 13, 14]; // RGB

fn part1(input: &str) -> String {
    let mut sum = 0;
    let games = parse_input(input);

    for game in games {
        if game.is_possible() {
            sum += game.id;
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    let games = parse_input(input);

    for game in games {
        let fewest_cubes = game.fewest_cubes();
        sum += fewest_cubes.iter().product::<u32>();
    }

    sum.to_string()
}

struct Game {
    id: u32,
    draws: Vec<[u32; 3]>, // RGB
}
impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            draws: Vec::new(),
        }
    }

    fn add_draw(&mut self, draw: [u32; 3]) {
        self.draws.push(draw);
    }

    fn is_possible(&self) -> bool {
        for draw in &self.draws {
            for (i, val) in draw.iter().enumerate() {
                if *val > CUBE_AMOUNTS[i] {
                    return false;
                }
            }
        }

        true
    }

    fn fewest_cubes(&self) -> [u32; 3] {
        let mut fewest_cubes = [0; 3];

        for draw in &self.draws {
            for (i, val) in draw.iter().enumerate() {
                if *val > fewest_cubes[i] {
                    fewest_cubes[i] = *val;
                }
            }
        }

        fewest_cubes
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.trim().lines() {
        let (game_id, draws) = line.split_once(':').unwrap();
        let game_id = game_id.trim_start_matches("Game ").parse::<u32>().unwrap();

        let mut game = Game::new(game_id);
        for draw in draws.split(';') {
            let mut values = [0; 3];

            for value in draw.split(',') {
                let val_num = value
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                if value.contains("green") {
                    values[1] = val_num;
                } else if value.contains("blue") {
                    values[2] = val_num;
                } else if value.contains("red") {
                    values[0] = val_num;
                } else {
                    panic!("Invalid color!");
                }
            }

            game.add_draw(values);
        }

        games.push(game);
    }

    games
}
