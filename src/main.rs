mod test;
mod fizzbuzz;

/// this is a test macro
macro_rules! test_macro {
    () => {
        println!("test macro");
    };
}

fn main() {
    // use test module
    test::test_print();

    // fizzbuzz
    fizzbuzz::fizzbuzz_to(100);

    println!("Hello World!");

    // testing a macro
    test_macro!();

    // debug format?
    println!("hello, {:?} ?", "testing");

    // change a mutable boolean
    let mut _test_bool: bool = true;
    _test_bool = false;

    // suffix annotation
    let _test_float = 0.32f32;
    // default annotation
    let _test_float_two: f64 = 0.64;

    let mut _my_list: Vec<i32> = vec![23, 65, 34, 88, 75, 9, 12];
    for i in _my_list {
        println!("{:?}", i);
    }

    //my_list.push(91);

    let _test_list: Vec<i32> = Vec::with_capacity(11);
}
