struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

struct NoShape;
impl NoShape {
    fn area(&self) -> u32 {
        return 0;
    }
    fn perimeter(&self) -> u32 {
        return 0;
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User 1 username: {}", user1.username);
    println!("User 1 active: {}", user1.active);
    println!("User 1 email: {}", user1.email);
    println!("User 1 sign_in_count: {}", user1.sign_in_count);

    let rect1 = Rect {
        width: 4,
        height: 5,
    };
    println!("area of reactangle is {}", rect1.area());
    println!("perimeter of reactangle is {}", rect1.perimeter());

    let no_shape1 = NoShape;
    println!("area of no shape is {}", no_shape1.area());
    println!("perimeter of no shape is {}", no_shape1.perimeter());

}
