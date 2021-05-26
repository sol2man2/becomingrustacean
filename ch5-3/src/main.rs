fn main() {
    println!("ch5-3!");

    // Defining Methods
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Where’s the -> Operator?
    // . operator vs -> operator

    // .
    // clazz y;
    // y.method();

    // ->
    // clazz* x = new clazz();
    // x->method();
    // (*x)->method();

    // automatic referencing and dereferencing
    // rust는 호출에 따라 자동으로 &, &mut나 *를 추가한다.
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // Methods with More Parameters
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated Functions
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    let sq = Rectangle::square(3);

    // Multiple impl Blocks
}
