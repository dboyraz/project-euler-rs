const UNITS: [usize; 10]     = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4]; // 0..9
const TEENS: [usize; 10]     = [3, 6, 6, 8, 8, 7, 7, 9, 8, 8]; // 10..19
const TENS:  [usize; 10]     = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6]; // 20..90
const HUNDRED: usize         = 7;  // "hundred"
const AND: usize             = 3;  // "and"
const ONE_THOUSAND: usize    = 11; // "onethousand"

fn word_len(n: usize) -> usize {
    match n {
        1..=9    => UNITS[n],
        10..=19  => TEENS[n - 10],
        20..=99  => TENS[n / 10] + UNITS[n % 10],
        100..=999 => {
            let hundreds = UNITS[n / 100] + HUNDRED;
            let rem = n % 100;
            if rem == 0 {
                hundreds
            } else {
                hundreds + AND + word_len(rem)
            }
        }
        1000 => ONE_THOUSAND,
        _ => 0,
    }
}

fn main() {
    let total: usize = (1..=1000).map(word_len).sum();
    println!("{total}");
}
