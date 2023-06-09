fn main() {
    let factors = prime_factors(600851475143);
    let max = factors.iter().max().unwrap();
    println!("{:?}", max);
}

fn prime_factors(nr: u64) -> Vec<u64> {
    let max = (nr as f64).sqrt() as u64;
    let mut factors: Vec<u64> = vec![];
    let mut remaining = nr;

    while remaining > 1 {
        for n in 2..max {
            if remaining % n == 0 {
                remaining /= n;
                factors.push(n);
                break;
            }
        }
    }

    return factors;
}
