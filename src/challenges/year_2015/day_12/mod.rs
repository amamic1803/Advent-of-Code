use crate::challenges::Day;
use serde_json::{from_str, Value};

pub(crate) fn day_12() -> Day {
    Day::new (
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    let json_val: Value = from_str(input).unwrap();
    println!("{}", sum_ints(&json_val));
}

fn part2(input: &str) {
    let json_val: Value = from_str(input).unwrap();
    println!("{}", sum_ints_wo_red(&json_val));
}

fn sum_ints(json_val: &Value) -> i64 {
    match json_val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(arr) => arr.iter().map(sum_ints).sum(),
        Value::Object(obj) => obj.iter().map(|x| sum_ints(x.1)).sum(),
    }
}

fn sum_ints_wo_red(json_val: &Value) -> i64 {
    match json_val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(arr) => arr.iter().map(sum_ints_wo_red).sum(),
        Value::Object(obj) => {
            if obj.iter().any(|x| x.1 == "red") {
                0
            } else {
                obj.iter().map(|x| sum_ints_wo_red(x.1)).sum()
            }
        },
    }
}