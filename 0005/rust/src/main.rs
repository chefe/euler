fn main() {
    let mut nr = 10;

    while is_evenly_divisible(nr) == false {
        nr += 1;
    }

    println!("{}", nr);
}

fn is_evenly_divisible(nr: u32) -> bool {
    for n in 2..21 {
        if nr % n != 0 {
            return false;
        }
    }

    return true;
}
