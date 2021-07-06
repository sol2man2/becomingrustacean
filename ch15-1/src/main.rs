fn main() {
    println!("ch15-1!");
    // Using Box<T> to Point to Data on the Heap
    // You’ll use them most often in these situations:
    // 컴파일 타임에 크기를 알 수 없고 맥락에서 값을 사용하고 싶을 때
    // 대량의 데이터를 가지고 사용시 복사가 아닌 소유권을 넘기길 원할 때
    // 값을 소유하길 원하고 전용 타입보다는 특정 형질을 구현한 타입에 관심있을 때

    // Using a Box<T> to Store Data on the Heap
    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes
    // More Information About the Cons List
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // use crate::List::{Cons, Nil};
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // Computing the Size of a Non-Recursive Type
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    // use crate::List::{Cons, Nil};
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
