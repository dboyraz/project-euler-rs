use std::fs;

fn main() -> std::io::Result<()> {
    let s = fs::read_to_string("src/bin/p22/names.txt")?;
    let mut names: Vec<String> = s
        .split(',')
        .map(|t| t.trim().trim_matches('"').to_string())
        .collect();

    names.sort();

    let values: Vec<u32> = names.iter().map(|n| name_value(n)).collect();

    

    
    let weighted: Vec<u64> = values
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as u64 + 1) * v as u64)
        .collect();

    let total: u64 = weighted.iter().sum();

    println!("{}",total);

    Ok(())
}

fn name_value(name: &str) -> u32 {
    name.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c.to_ascii_uppercase() as u8 - b'A' + 1) as u32)
        .sum()
}
