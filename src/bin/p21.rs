use std::time::Instant;

fn main(){
    println!("Starting calculation...");
    
    let start = Instant::now();
    let result = sum_amicable_under_10k();
    let duration = start.elapsed();
    
    println!("sum of all amicable numbers under 10k: {}", result);
    println!("Time elapsed: {:.2?}", duration);
    println!("Time elapsed: {} microseconds", duration.as_micros());
}


fn sum_amicable_under_10k() -> usize {
    let limit = 10_000;

    // s[n] = sum of proper divisors of n
    let mut s = vec![0usize; limit + 1];

    // Sieve: add each divisor d to multiples m = 2d, 3d, ... <= limit
    for d in 1..=limit / 2 {
        let mut m = d * 2;
        while m <= limit {
            s[m] += d;
            m += d;
        }
    }

    // Sum each amicable pair (a, b) once using a < b
    let mut sum = 0usize;
    for a in 2..=limit {
        let b = s[a];
        if b > a && b <= limit && s[b] == a && b != a {
            sum += a + b;
        }
    }

    sum
}
