fn main() {
    // IF - ELSE
    let is_even = true;

    if is_even {
        println!("the number is even");
    } else if !is_even {
        println!("the number is NOT even");
    }

    // LOOPS
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("");

    let sentence = String::from("My name is Krishna");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
