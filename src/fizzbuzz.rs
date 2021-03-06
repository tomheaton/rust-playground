/// check if a number is divisible by another
fn is_divisible_by(num: u32, den: u32) -> bool {
    if den == 0 {
        return false;
    }
    num % den == 0
}

/// fizzbuzz for a number n
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

/// perform fizzbuzz to a number n
pub fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
