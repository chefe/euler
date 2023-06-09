fn main() {
    let mut sum = 0_u64;

    for n in 2_u64..2_000_000 {
        if is_prime(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn is_prime(nr: u64) -> bool {
    for n in 2..((nr / 2) + 1) {
        if nr % n == 0 {
            return false;
        }
    }

    return true;
}
