//! A module for shared functionality between challenges.

pub mod graph;
pub mod structures;

/// Find the solution to a system of congruences using the Chinese Remainder Theorem.
/// For a system of congruences:
/// x ≡ a1 (mod m1)
/// x ≡ a2 (mod m2)
/// ...
/// x ≡ an (mod mn)
/// Where m1, m2, ..., mn are pairwise coprime, the solution x is unique modulo M, where M = m1 * m2 * ... * mn.
/// # Arguments
/// * `congruences` - The congruences as a slice of tuples. Each tuple contains the (remainder, modulus). The slice should be sorted by modulus in descending order for the best performance.
/// # Returns
/// * `u64` - The solution to the system of congruences.
/// # Panics
/// Panics if there are no congruences.
pub fn chinese_remainder_theorem(congruences: &[(u64, u64)]) -> u64 {
    match congruences.len() {
        0 => panic!("There must be at least 1 congruence."),
        1 => congruences[0].0,
        _ => {
            let (mut solution, mut modulus) = congruences[0];

            for congruence in congruences.iter().skip(1) {
                while solution % congruence.1 != congruence.0 {
                    solution += modulus;
                }
                modulus *= congruence.1;
            }

            solution
        }
    }
}

/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u64` - The greatest common divisor.
pub fn gcd(mut num1: u64, mut num2: u64) -> u64 {
    if num1 < num2 {
        (num1, num2) = (num2, num1);
    }
    while num2 != 0 {
        (num1, num2) = (num2, num1 % num2);
    }
    num1
}

/// Finds the greatest common divisor of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * `u64` - The greatest common divisor.
pub fn gcd_multiple(nums: &[u64]) -> u64 {
    assert!(nums.len() > 1, "There must be at least 2 numbers.");
    let mut result = gcd(nums[0], nums[1]);
    for n in nums.iter().skip(2) {
        result = gcd(result, *n);
    }
    result
}

/// Checks if a number is prime.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `u64` - The smallest divisor if the number is not prime, otherwise 1.
pub fn is_prime(num: u64) -> (bool, u64) {
    assert!(num >= 2, "Number must be greater than or equal to 2.");

    if num == 2 {
        return (true, 0);
    } else if num % 2 == 0 {
        return (false, 2);
    }

    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return (false, i);
        }
        i += 2;
    }

    (true, 1)
}

/// Finds the least common multiple of two numbers.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u64` - The least common multiple.
pub fn lcm(num1: u64, num2: u64) -> u64 {
    (num1 / gcd(num1, num2)) * num2
}

/// Finds the least common multiple of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * `u64` - The least common multiple.
pub fn lcm_multiple(nums: &[u64]) -> u64 {
    assert!(nums.len() > 1, "There must be at least 2 numbers.");
    let mut result = lcm(nums[0], nums[1]);
    for n in nums.iter().skip(2) {
        result = lcm(result, *n);
    }
    result
}

/// Finds the Manhattan distance between two locations.
/// Manhattan distance is the sum of the absolute differences of the x and y coordinates.
/// # Arguments
/// * `loc1` - The first location as a tuple of (x, y).
/// * `loc2` - The second location as a tuple of (x, y).
/// # Returns
/// * `u64` - The Manhattan distance.
pub fn manhattan_distance(loc1: (i64, i64), loc2: (i64, i64)) -> u64 {
    loc1.0.abs_diff(loc2.0) + loc1.1.abs_diff(loc2.1)
}
