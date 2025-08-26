fn main() {
    let limit = 1_000_000;
    let mut longest_chain = 0;
    let mut number_with_longest_chain = 0;

    for starting_number in 1..limit {
        let chain_length = get_chain_length(starting_number);

        if chain_length > longest_chain {
            longest_chain = chain_length;
            number_with_longest_chain = starting_number;
        }
    }

    println!("Number with longest chain: {}", number_with_longest_chain);
    println!("Chain length: {}", longest_chain);
}

fn get_chain_length(mut n: u64) -> u32 {
    let mut chain_length = 1;

    while n != 1 {
        n = collatz_seq(n);
        chain_length += 1;
    }

    chain_length
}

fn collatz_seq(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}
