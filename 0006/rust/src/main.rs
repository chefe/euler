fn main() {
    let mut sum_of_squares = 0_u32;
    let mut sum_of_numbers = 0_u32;

    for n in 1..101 {
        sum_of_numbers += n;
        sum_of_squares += n * n
    }

    let square_of_sum = sum_of_numbers.pow(2);

    println!(
        "{} - {} = {}",
        square_of_sum,
        sum_of_squares,
        square_of_sum - sum_of_squares
    );
}
