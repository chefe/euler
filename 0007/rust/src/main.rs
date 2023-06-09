fn main() {
    let mut nr = 1;
    let mut prime_count = 0;

    while prime_count < 10001 {
        nr += 1;

        if is_prime(nr) {
            prime_count += 1;
        }
    }

    println!("{}", nr);
}

fn is_prime(nr: u32) -> bool {
    for n in 2..nr {
        if nr % n == 0 {
            return false;
        }
    }

    return true;
}
