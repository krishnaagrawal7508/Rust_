fn main() {

    // NUMBERS & FLOATS
    let signed_number: i32 = -500;
    let unsigned_number: u32 = 100;
    let float_number: f32 = 10.11;

    println!(
        "x: {}, y: {}, z: {}",
        signed_number, unsigned_number, float_number
    );

    // BOOLEAN
    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male && is_above_18 {
        println!("you are a legal male");
    } else if is_male {
        println!("you are a male");
    } else {
        println!("you are not a male")
    }


    // STRINGS
    let greeting: String = String::from("krishna");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(0);
    // unwrap is not good, it tell compiler that we are okay with runtime errors
    // here an error could be if we ask for character at 1000 index which doesn't exist and will throw an error
    println!("char1: {}", char1.unwrap());

    // Right way to do it is
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at this index"),
    }

}
