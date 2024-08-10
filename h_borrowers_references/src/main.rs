// there can be multiple non-mutable borrowers
// but there can be only one borrower that is mutable, in that case no non mutable borrowers also cant be there

fn main() {
    // #1
    let s1 = String::from("hello");
    let s2 = &s1; // borrower
    println!("{}", s1);
    println!("{}", s2); // This is valid, The first pointer wasn't invalidated

    // #2
    takes_ownership(&s1);
    println!("{}", s1); // This is valid because ownership was not transferred

    // #3
    let mut s3 = String::from("Hello");
    update_word(&mut s3);
    println!("{}", s3);

    // #4
    let mut owner = String::from("hello #4");
    let borrow_1 = &mut owner;
    // cannot do these:
    // let borrow_2 = &mut owner;
    // let borrow_3 = & owner
    println!("{}", borrow_1);
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string); // some_string is borrowed and not moved
}

fn update_word(word: &mut String) {
    word.push_str(" World #3");
    println!("{}", word);
}
