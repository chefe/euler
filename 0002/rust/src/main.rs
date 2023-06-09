fn main() {
    let mut sum = 0;

    let mut a = 1;
    let mut b = 2;

    loop {
        if a % 2 == 0 {
            sum += a;
        }

        let c = a + b;
        a = b;
        b = c;

        if a > 4_000_000 {
            break;
        }
    }

    println!("{}", sum);
}
