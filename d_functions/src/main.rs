fn main() {
    let a = 10;
    let b = 12;

    let sum = do_sum(a, b);

    println!("{}", sum);
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
