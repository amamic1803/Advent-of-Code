use std::cmp::Ordering;
use crate::shared::structures::Day;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use regex::Regex;
use smallvec::SmallVec;

pub fn day_11() -> Day {
    Day::new(11, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    // get initial building state
    let mut area = parse_input(input);
    area.floors.iter_mut().for_each(|floor| floor.sort());

    // setup wanted building state
    let mut wanted_state = Area::new();
    wanted_state.elevator = 3;
    area.floors.iter().for_each(|floor| wanted_state.floors[3].extend(floor.iter().copied()));
    wanted_state.floors.iter_mut().for_each(|floor| floor.sort());

    // search for optimal solution
    if area != wanted_state {
        bfs_search(area, wanted_state).to_string()
    } else {
        0.to_string()
    }
}

fn part2(input: &str) -> String {
    // get initial building state
    let mut area = parse_input(input);
    area.floors[0].push(Element { id: u8::MAX, is_generator: false });
    area.floors[0].push(Element { id: u8::MAX, is_generator: true });
    area.floors[0].push(Element { id: u8::MAX - 1, is_generator: false });
    area.floors[0].push(Element { id: u8::MAX - 1, is_generator: true });
    area.floors.iter_mut().for_each(|floor| floor.sort());

    // setup wanted building state
    let mut wanted_state = Area::new();
    wanted_state.elevator = 3;
    area.floors.iter().for_each(|floor| wanted_state.floors[3].extend(floor.iter().copied()));
    wanted_state.floors.iter_mut().for_each(|floor| floor.sort());

    // search for optimal solution
    if area != wanted_state {
        bfs_search(area, wanted_state).to_string()
    } else {
        0.to_string()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Area {
    elevator: u8,
    floors: [SmallVec<[Element; 14]>; 4],
}
impl Area {
    fn new() -> Self {
        Area {
            elevator: 0,
            floors: [
                SmallVec::with_capacity(14),
                SmallVec::with_capacity(14),
                SmallVec::with_capacity(14),
                SmallVec::with_capacity(14),
            ],
        }
    }
    
    fn is_valid(&self) -> bool {
        if self.floors[self.elevator as usize].is_empty() {
            return false;
        }
        for floor in &self.floors {
            // if there are no generators, then floor must be valid
            if floor.iter().any(|element| element.is_generator) {
                for element in floor {
                    if !element.is_generator {
                        let mut element_generator = *element;
                        element_generator.is_generator = true;
                        if !floor.contains(&element_generator) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

/// Element representing microchip or generator
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Element {
    id: u8,              // id of the type of element
    is_generator: bool,  // true if element is a generator, false if it is a microchip
}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut cmp_result = self.id.cmp(&other.id);
        if let Ordering::Equal = cmp_result {
            cmp_result = self.is_generator.cmp(&other.is_generator);
        }
        cmp_result
    }
}

/// Generate initial building state from input
fn parse_input(input: &str) -> Area {
    let mut id_map = HashMap::new();
    let mut area = Area::new();

    let re = Regex::new(r", and|and|,").unwrap();
    for (i, floor_line) in input.lines().enumerate() {
        if floor_line.contains("nothing relevant") {
            continue;
        }
        re.split(floor_line
            .trim_end_matches('.')
            .split_once(" contains ")
            .unwrap()
            .1)
            .for_each(|element| {
                let mut element = element.trim().trim_start_matches("a ");
                let mut is_generator = false;
                if element.ends_with("generator") {
                    is_generator = true;
                    element = element.trim_end_matches(" generator");
                } else {
                    element = element.trim_end_matches("-compatible microchip");
                }
                let len = id_map.len() as u8;
                let id = *id_map.entry(element).or_insert(len);
                area.floors[i].push(Element { id, is_generator });
            });
    }

    area
}

/// Search for the minimum number of steps to get from initial_state to wanted_state using BFS algorithm
/// Panics if no solution is found, that is there isn't any way to achieve wanted_state starting from initial_state.
fn bfs_search(initial_state: Area, wanted_state: Area) -> i32 {
    let mut state_step_map = HashMap::new();
    let mut queue = VecDeque::new();
    state_step_map.insert(initial_state.clone(), 0);
    queue.push_back(initial_state);

    let mut up_down_moves = Vec::with_capacity(2);
    while let Some(state) = queue.pop_front() {
        up_down_moves.clear();
        if state.elevator > 0 {
            up_down_moves.push(-1);
        }
        if state.elevator < 3 {
            up_down_moves.push(1);
        }

        for &k in &up_down_moves {
            for i in 0..state.floors[state.elevator as usize].len() {
                // moving 1 element
                let mut next_state = state.clone();
                let removed = next_state.floors[next_state.elevator as usize].remove(i);
                next_state.elevator = next_state.elevator.wrapping_add_signed(k);
                next_state.floors[next_state.elevator as usize].push(removed);
                next_state.floors[next_state.elevator as usize].sort();
                
                if !state_step_map.contains_key(&next_state) && next_state.is_valid() {
                    state_step_map.insert(next_state.clone(), state_step_map.get(&state).unwrap() + 1);
                    if next_state == wanted_state {
                        return *state_step_map.get(&next_state).unwrap();
                    } else {
                        queue.push_back(next_state);
                    }
                }

                // moving 2 elements
                for j in (i + 1)..state.floors[state.elevator as usize].len() {
                    let mut next_state = state.clone();
                    let removed1 = next_state.floors[next_state.elevator as usize].remove(j);
                    let removed2 = next_state.floors[next_state.elevator as usize].remove(i);
                    next_state.elevator = next_state.elevator.wrapping_add_signed(k);
                    next_state.floors[next_state.elevator as usize].push(removed1);
                    next_state.floors[next_state.elevator as usize].push(removed2);
                    next_state.floors[next_state.elevator as usize].sort();
                    
                    if !state_step_map.contains_key(&next_state) && next_state.is_valid() {
                        state_step_map.insert(next_state.clone(), state_step_map.get(&state).unwrap() + 1);
                        if next_state == wanted_state {
                            return *state_step_map.get(&next_state).unwrap();
                        } else {
                            queue.push_back(next_state);
                        }
                    }
                }
            }
        }
    }

    panic!("No solution found");
}
