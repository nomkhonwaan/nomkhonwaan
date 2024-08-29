use regex::Regex;
use std::str::FromStr;

/// 592. Fraction Addition and Subtraction
/// https://leetcode.com/problems/fraction-addition-and-subtraction/description/
struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let expression = fill_zero(expression);
        let fractions = split_fractions(&expression);
        let operators = split_operators(&expression);

        // get the first fraction from the list
        let mut fraction = fractions[0];

        // perform fraction operations
        for (i, operator) in operators.into_iter().enumerate() {
            match operator.as_str() {
                "-" => fraction.sub(&fractions[i + 1]),
                "+" => fraction.add(&fractions[i + 1]),
                _ => panic!("invalid operator"),
            }
        }

        // find the greatest common divisor
        let g = gcd(fraction.numerator, fraction.denominator);

        format!("{}/{}", fraction.numerator / g, fraction.denominator / g)
    }
}

/// Fill the zero value (in fraction) when the expression starts with negative operator (-).
fn fill_zero(expression: String) -> String {
    if expression.starts_with("-") {
        return format!("0/1{}", expression);
    }
    expression
}

/// Split an expression string into vector of Fraction s.
fn split_fractions(expression: &str) -> Vec<Fraction> {
    Regex::new(r"[-|+]")
        .unwrap()
        .split(&expression)
        .into_iter()
        .map(|s| Fraction::from_str(s).unwrap())
        .collect()
}

/// Split an expression string into vector of operators.
fn split_operators(expression: &str) -> Vec<String> {
    Regex::new(r"[0-9|\/]")
        .unwrap()
        .replace_all(&expression, "") // remove all fractions from the string
        .split("") // now, only operators are remain, split them all
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Find the greatest common divisor of the fraction.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    // GCD must not negative!
    a.abs()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn add(&mut self, other: &Fraction) {
        self.numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        self.denominator = self.denominator * other.denominator;
    }

    fn sub(&mut self, other: &Fraction) {
        self.numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        self.denominator = self.denominator * other.denominator;
    }
}

impl FromStr for Fraction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (numerator, denominator) = match s.split_once("/") {
            Some((n, d)) => (n.parse::<i32>()?, d.parse::<i32>()?),
            _ => panic!("invalid fraction"),
        };

        Ok(Fraction {
            numerator,
            denominator,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_addition() {
        assert_eq!("0/1", Solution::fraction_addition("-1/2+1/2".to_string()));
        assert_eq!(
            "1/3",
            Solution::fraction_addition("-1/2+1/2+1/3".to_string())
        );
        assert_eq!("-1/6", Solution::fraction_addition("1/3-1/2".to_string()));
        assert_eq!(
            "68/15",
            Solution::fraction_addition("7/3+5/2-3/10".to_string())
        );
        assert_eq!(
            "-37/60",
            Solution::fraction_addition("1/3-5/4+3/10".to_string())
        );
        assert_eq!(
            "-1189/280",
            Solution::fraction_addition("4/1+6/5-1/8-9/2-1/1+6/7-10/7-3/4-3/2-1/1".to_string())
        );
        assert_eq!(
            "-13/10",
            Solution::fraction_addition("-1/4-4/5-1/4".to_string())
        );
    }

    #[test]
    fn test_fill_zero() {
        assert_eq!("0/1-1/2", fill_zero("-1/2".to_string()));
    }

    #[test]
    fn test_split_fractions() {
        assert_eq!(
            vec![
                Fraction {
                    numerator: 0,
                    denominator: 1
                },
                Fraction {
                    numerator: 1,
                    denominator: 2
                }
            ],
            split_fractions("0/1-1/2")
        );
    }

    #[test]
    fn test_split_operators() {
        assert_eq!(vec!["-", "+"], split_operators("0/1-1/2+5/9"))
    }

    #[test]
    fn test_gcd() {
        assert_eq!(14, gcd(42, 56));
    }
}
