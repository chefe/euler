fn main() {
    for a in 0_u32..1000 {
        for b in (a + 1)..1000 {
            for c in (b + 1)..1000 {
                if a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2) {
                    println!("{}", a * b * c);
                }
            }
        }
    }
}
