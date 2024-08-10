// generic type is of any type, denoted by capital letters
struct Point<A, B> {
    x: A,
    y: B,
    z: B,
}

fn main() {
    let p1 = Point { x: 5, y: 5, z: 5 };
    let p2 = Point {
        x: 5,
        y: "agrawal",
        z: "krishna",
    };
    let p3 = Point {
        x: "krishna",
        y: 1,
        z: 2,
    };

    println!("Integer Point: ({}, {}, {})", p1.x, p1.y, p1.z);
    println!("Integer Point: ({}, {}, {})", p2.x, p2.y, p2.z);
    println!("Integer Point: ({}, {}, {})", p3.x, p3.y, p3.z);
}
