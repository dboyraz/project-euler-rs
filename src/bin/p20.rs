use num_bigint::BigUint;

fn main() {
    // Calculate 100! using BigUint
    let factorial_100 = factorial(100);
    
    // Sum the digits
    let digit_sum = sum_of_digits(&factorial_100);
    
    println!("Sum of digits in 100!: {}", digit_sum);
}

fn factorial(n: u32) -> BigUint {
    (1..=n).map(BigUint::from).product()
}

fn sum_of_digits(num: &BigUint) -> u32 {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}
