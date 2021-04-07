fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of rectangle is {}", area(width1, height1));

    let rect1 = (30, 50);

    println!("The area of rectangle is {}", area1(rect1));

    let rect2 = Rect {
        width: 30,
        height: 50,
    };

    println!("rect is {:?}", rect2);

    println!("The area of rectangle is {}", area2(&rect2));
    println!("The area of rectangle is {}", rect2.area());

    let rect3 = Rect {
        width: 10,
        height: 40,
    };

    let rect4 = Rect {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    let sq = Rect::square(5);

    let mut s = String::new();
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        if other.width < self.width && other.height < self.height {
            return true;
        }
        return false;
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rect) -> u32 {
    rect.width * rect.height
}
