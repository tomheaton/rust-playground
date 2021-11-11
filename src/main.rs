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

    let mut my_list: Vec<i32> = vec![23, 65, 34, 88, 75, 9, 12];
    for i in my_list {
        println!("{:?}", i);
    }

    //my_list.push(91);

    let test_list: Vec<i32> = Vec::with_capacity(11);
}
