
// 生成斐波拉契数组
fn main() {
    let mut n: u32 = 0;

    while n < 10 {
        print!("{0} ", get_fibonacci_sequence(n));
        n += 1;
    }
    println!("");
}

/// F(n) 
fn get_fibonacci_sequence(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    get_fibonacci_sequence(n - 1) + get_fibonacci_sequence(n - 2)
}
