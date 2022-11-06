#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    // 在impl中的都称作关联函数，第一个参数为self, &self, mut self, &mut self 的称为方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 1,
        height: 45,
    };

    println!("{:#?}", rect);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        ..rect // rec余下的都直接转移过来，rect失效
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.get_area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 70,
        height: 45,
    };

    let squar = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold squar? {}", rect1.can_hold(&squar));
}
