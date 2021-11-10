/// this is a test macro
macro_rules! test_macro {
    () => {
        println!("test macro");
    };
}

fn main() {
    println!("Hello World!");

    // testing a macro
    test_macro!();

    // debug format?
    println!("hello, {:?} ?", "testing");

    // change a mutable boolean
    let mut test_bool: bool = true;
    test_bool = false;

    // suffix annotation
    let test_float = 0.32f32;
    // default annotation
    let test_float_two: f64 = 0.64;
}
