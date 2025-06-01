use crate::{Day, Error};
use serde_json::{Value, from_str};

pub struct Day12;
impl Day12 {
    pub fn new() -> Self {
        Self
    }

    fn sum_ints(json_val: &Value) -> i64 {
        match json_val {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(n) => n.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(arr) => arr.iter().map(Self::sum_ints).sum(),
            Value::Object(obj) => obj.iter().map(|x| Self::sum_ints(x.1)).sum(),
        }
    }

    fn sum_ints_wo_red(json_val: &Value) -> i64 {
        match json_val {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(n) => n.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(arr) => arr.iter().map(Self::sum_ints_wo_red).sum(),
            Value::Object(obj) => {
                if obj.iter().any(|x| x.1 == "red") {
                    0
                } else {
                    obj.iter().map(|x| Self::sum_ints_wo_red(x.1)).sum()
                }
            }
        }
    }
}
impl Day for Day12 {
    fn id(&self) -> usize {
        12
    }
    fn title(&self) -> &str {
        "JSAbacusFramework.io"
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let json_val = from_str(input).unwrap();
        Ok(Self::sum_ints(&json_val).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let json_val = from_str(input).unwrap();
        Ok(Self::sum_ints_wo_red(&json_val).to_string())
    }
}
