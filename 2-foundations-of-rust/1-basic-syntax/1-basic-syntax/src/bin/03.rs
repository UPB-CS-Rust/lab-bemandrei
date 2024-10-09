fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut x = 0;
    let mut y = 100;
    for i in 0..8{
        if input[i] > x{
            x = input[i]
        }
        if input[i] < y{
            y = input[i]
        }
    }
    println!("{} is largest and {} is smallest", y, x);
}
