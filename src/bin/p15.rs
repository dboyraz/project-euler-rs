fn main() {
    let grid_size = 20;
    let routes = lattice_paths(grid_size);
    
    println!("Number of routes through a {}x{} grid: {}", grid_size, grid_size, routes);
}

fn lattice_paths(n: u64) -> u64 {
    
    // use the multiplicative formula to avoid large factorials:
    // C(2n, n) = (2n * (2n-1) * ... * (n+1)) / (n * (n-1) * ... * 1)
    
    let mut result = 1u64;
    
    for i in 1..=n {
        result = result * (n + i) / i;
    }
    
    result
}