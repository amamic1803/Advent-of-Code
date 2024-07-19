//! Mathematical functions

use std::borrow::Borrow;
use num_traits::{ConstZero, PrimInt, Unsigned};

/// Find the solution to a system of congruences using the Chinese Remainder Theorem.
/// For a system of congruences:
/// x ≡ a1 (mod m1)
/// x ≡ a2 (mod m2)
/// ...
/// x ≡ an (mod mn)
/// Where m1, m2, ..., mn are pairwise coprime, the solution x is unique modulo M, where M = m1 * m2 * ... * mn.
/// # Arguments
/// * `congruences` - The congruences (tuples). Each tuple contains the (remainder, modulus). The slice should be sorted by modulus in descending order for the best performance.
/// # Returns
/// * `u64` - The solution to the system of congruences.
/// # Panics
/// If there are no congruences. There must be at least 1 congruence.
/// Note that this function will run infinitely if the moduli are not pairwise coprime.
pub fn chinese_remainder_theorem<T, U>(congruences: U) -> u64
where
    T: Borrow<(u64, u64)>,
    U: IntoIterator<Item = T>,
{
    let mut congruences = congruences.into_iter();
    let (mut solution, mut modulus) = congruences.next().expect("There must be at least 1 congruence.").borrow();
    
    for congruence in congruences {
        let (remainder, modulo) = congruence.borrow();
        while solution % modulo != *remainder {
            solution += modulus;
        }
        modulus *= modulo;
    }
    
    solution
}

/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * The greatest common divisor.
pub fn gcd<T>(mut num1: T, mut num2: T) -> T
where
    T: PrimInt + Unsigned + ConstZero,
{
    if num1 < num2 {
        (num1, num2) = (num2, num1);
    }
    while num2 != T::ZERO {
        (num1, num2) = (num2, num1 % num2);
    }
    num1
}

/// Finds the greatest common divisor of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * The greatest common divisor.
/// # Panics
/// If there are less than 2 numbers.
pub fn gcd_multiple<T, U, I>(nums: I) -> T
where
    T: PrimInt + Unsigned + ConstZero,
    U: Borrow<T>,
    I: IntoIterator<Item = U>,
{
    let mut nums = nums.into_iter();
    let n1 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let n2 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let mut result = gcd(n1, n2);
    for n in nums {
        result = gcd(result, *n.borrow());
    }
    result
}

/// Checks if a number is prime.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `u64` - The smallest divisor if the number is not prime, otherwise 1.
pub fn is_prime(n: u64) -> (bool, u64) {
    assert!(n >= 2, "Number must be greater than or equal to 2.");

    if n == 2 || n == 3 {
        (true, 1)
    } else if n % 2 == 0 {
        (false, 2)
    } else if n % 3 == 0 {
        (false, 3)
    } else {
        for i in (5..=((n as f64).sqrt().floor() as u64)).step_by(6) {
            if n % i == 0 {
                return (false, i);
            } else if n % (i + 2) == 0 {
                return (false, i + 2);
            }
        }

        (true, 1)
    }
}

/// Finds the least common multiple of two numbers.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * The least common multiple.
pub fn lcm<T>(num1: T, num2: T) -> T
where
    T: PrimInt + Unsigned + ConstZero,
{
    (num1 / gcd(num1, num2)) * num2
}

/// Finds the least common multiple of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * The least common multiple.
/// # Panics
/// If there are less than 2 numbers.
pub fn lcm_multiple<T, U, I>(nums: I) -> T
where
    T: PrimInt + Unsigned + ConstZero,
    U: Borrow<T>,
    I: IntoIterator<Item = U>,
{
    let mut nums = nums.into_iter();
    let n1 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let n2 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let mut result = lcm(n1, n2);
    for n in nums {
        result = lcm(result, *n.borrow());
    }
    result
}

/// Finds the Manhattan distance between two locations.
/// Manhattan distance is the sum of the absolute differences of x and y coordinates.
/// # Arguments
/// * `loc1` - The first location as a tuple of (x, y).
/// * `loc2` - The second location as a tuple of (x, y).
/// # Returns
/// * `u64` - The Manhattan distance.
pub fn manhattan_distance(loc1: (i64, i64), loc2: (i64, i64)) -> u64 {
    loc1.0.abs_diff(loc2.0) + loc1.1.abs_diff(loc2.1)
}
