use crate::shared::structures::Day;

pub fn day_14() -> Day {
    Day::new(14, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let reindeers = parse_input(input);
    reindeers.iter().map(|r| r.distance(2503)).max().unwrap().to_string()
}

fn part2(input: &str) -> String {
    let reindeers = parse_input(input);
    let mut scores: Vec<usize> = vec![0; reindeers.len()];
    let mut temp_distances: Vec<usize> = vec![0; reindeers.len()];

    for second in 1..=2503 {
        for (i, reindeer) in reindeers.iter().enumerate() {
            temp_distances[i] = reindeer.distance(second);
        }

        let max_distance = temp_distances.iter().max().unwrap();
        for (i, distance) in temp_distances.iter().enumerate() {
            if distance == max_distance {
                scores[i] += 1;
            }
        }
    }

    scores.iter().max().unwrap().to_string()
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let mut reindeers = Vec::new();

    for line in input.trim().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let speed = words[3].parse::<usize>().unwrap();
        let fly_time = words[6].parse::<usize>().unwrap();
        let rest_time = words[13].parse::<usize>().unwrap();

        reindeers.push(Reindeer::new(speed, fly_time, rest_time));
    }

    reindeers
}

struct Reindeer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn new(speed: usize, fly_time: usize, rest_time: usize) -> Self {
        Self { speed, fly_time, rest_time }
    }

    fn distance(&self, time: usize) -> usize {
        let cycle_time = self.fly_time + self.rest_time;
        let full_cycles = time / cycle_time;
        let leftover_time = time % cycle_time;
        let leftover_fly_time = if leftover_time > self.fly_time { self.fly_time } else { leftover_time };

        (full_cycles * self.fly_time + leftover_fly_time) * self.speed
    }
}
