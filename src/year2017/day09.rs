use crate::{Day, Error};

pub struct Day09;
impl Day09 {
    pub fn new() -> Self {
        Self
    }

    fn remove_cancelled_chars(input: &str) -> String {
        let mut stream = String::with_capacity(input.len());

        let mut input_iter = input.trim().chars();
        while let Some(c) = input_iter.next() {
            if c == '!' {
                input_iter.next();
            } else {
                stream.push(c);
            }
        }

        stream
    }
}
impl Day for Day09 {
    fn id(&self) -> usize {
        9
    }

    fn title(&self) -> &str {
        "Stream Processing"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let stream = Self::remove_cancelled_chars(input);

        let mut stack = Vec::new();
        let mut garbage = false;
        for c in stream.chars() {
            match c {
                '{' => {
                    if !garbage {
                        stack.push((c, stack.len() + 1));
                    }
                }
                '}' => {
                    if !garbage {
                        let (_, score) = stack.pop().unwrap();
                        match stack.last_mut() {
                            Some(last) => last.1 += score,
                            None => return Ok(score.to_string()),
                        }
                    }
                }
                '<' => garbage = true,
                '>' => garbage = false,
                _ => {}
            }
        }

        Err(Error::NoSolutionFound)
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let stream = Self::remove_cancelled_chars(input);
        let mut garbage_chars = 0;

        let mut garbage = false;
        for c in stream.chars() {
            match c {
                '<' => {
                    if garbage {
                        garbage_chars += 1;
                    } else {
                        garbage = true
                    }
                }
                '>' => garbage = false,
                _ => {
                    if garbage {
                        garbage_chars += 1;
                    }
                }
            }
        }

        Ok(garbage_chars.to_string())
    }
}
