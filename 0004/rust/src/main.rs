fn main() {
    let range = 1000;
    let mut max_nr = 0;

    for x in (1..range).rev() {
        for y in (1..x).rev() {
            let nr = x * y;
            if is_palindrome(nr) && nr > max_nr {
                max_nr = nr;
            }
        }
    }

    println!("{}", max_nr);
}

fn is_palindrome(value: u32) -> bool {
    let digits = (value as f32).log10().floor() as u32;

    for i in 0..((digits / 2) + 1) {
        if number_at(value, i) != number_at(value, digits - i) {
            return false;
        }
    }

    return true;
}

fn number_at(value: u32, position: u32) -> u32 {
    return (value / 10_u32.pow(position)) % 10;
}
