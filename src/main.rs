/// this is a test macro
macro_rules! test_macro {
    () => {
        println!("test macro");
    };
}

fn main() {
    println!("Hello World!");
    test_macro!();
    println!("hello, {:?} ?", "testing");
}
