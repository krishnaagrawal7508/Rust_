// In rust, all variables can have only one owner at a time only, if the owner dies the value also.

fn main() {
    // #1
    let x = 1; // crated on stack
    let y = 3; // created on stack
    println!("{}", sum(x, y));

    // #2
    let s1 = String::from("hello");
    let s2 = s1;

    // This line would cause a compile error because ownership has be changed to s2
    // s1 is removed/deleted from the heap
    // println!("{}", s1);
    println!("{}", s2);

    // to solve this - make two variable have the same value
    let s3 = s2.clone();
    println!("{}", s2);
    println!("{}", s3);

    // #3
    let my_string = String::from("hello");
    takes_ownership(my_string);
    // This line would cause a compile error because owner is chanegd
    // println!("{}", my_string);

    // to solve this - passing string to a function
    let random_string = String::from("solution 1");
    takes_ownership(random_string.clone());
    println!("{}", random_string);

    //OR
    let rand_string_1 = String::from("solution 2");
    let rand_string_2 = takes_ownership_safely(rand_string_1);
    println!("{}", rand_string_2);

    // OR
    let mut rand_string_ = String::from("solution 3");
    rand_string_ = takes_ownership_safely(rand_string_);
    println!("{}", rand_string_);

}

fn sum(a: i32, b: i32) -> i32 {
    // pass by value, new variable a,b,c is created in stack
    let c = a + b;
    return c;
}

fn takes_ownership(some_string: String) {
    // pass by reference
    println!("{}", some_string); // `some_string` now owns the data, my_string dies
}

fn takes_ownership_safely(some_string: String) -> String {
    // pass by reference
    println!("{}", some_string); // `some_string` now owns the data.
    return some_string;
}