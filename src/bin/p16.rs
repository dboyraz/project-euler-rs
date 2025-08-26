fn main() {
    let exp = 1000;
    let digit_sum = sum_of_digits_power_of_2(exp);
    println!("Sum of digits of 2^{}: {}", exp, digit_sum);
}

fn sum_of_digits_power_of_2(exp: u32) -> u32 {
    // Start with digits of 1 (2^0 = 1)
    let mut digits = vec![1];
    
    // Multiply by 2, exp times
    for _ in 0..exp {
        multiply_by_2(&mut digits);
    }
    
    // Sum all digits
    digits.iter().sum()
}

fn multiply_by_2(digits: &mut Vec<u32>) {
    let mut carry = 0;
    
    // Process each digit from right to left (least significant first)
    for digit in digits.iter_mut() {
        let product = *digit * 2 + carry;
        *digit = product % 10;  // Keep only the ones place
        carry = product / 10;   // Carry the tens place
    }
    
    // Add any remaining carry as new digits
    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10;
    }
}