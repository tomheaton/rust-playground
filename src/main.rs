#![allow(dead_code)]
#![allow(unused_macros)]

mod test;
mod fizzbuzz;
// use fizzbuzz::fizzbuzz_to;

/// this is a test macro
macro_rules! test_macro {
    () => {
        println!("test macro");
    };
}

#[derive(Debug)]
/// person struct
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Hello World!");

    // use test module
    // test::test_print();
    // test::test_types();

    // fizzbuzz
    // fizzbuzz::fizzbuzz_to(100);

    // testing a macro
    // test_macro!();

    let name = String::from("tom");
    let age = 19;
    let person = Person { name, age };

    // `:?` needed to debug print struct
    println!("{:?}", person);
}
