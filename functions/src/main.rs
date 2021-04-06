fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    let z = plus_one(five());
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
