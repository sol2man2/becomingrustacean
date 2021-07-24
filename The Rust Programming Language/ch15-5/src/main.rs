fn main() {
    println!("ch15-5!");
    // RefCell<T> and the Interior Mutability Pattern
    // Enforcing Borrowing Rules at Runtime with RefCell<T>
    // halting의 경우 borrowing을 검증하기 쉽지 않다
    // RefCell<T>는 borrowing에 안전하지만 compiler는 알 수 없는 경우
    // 1. Rc<T> enables multiple owners of the same data;
    // Box<T> and RefCell<T> have single owners.
    // 2. Box<T> allows immutable or mutable borrows checked at compile time;
    // Rc<T> allows only immutable borrows checked at compile time; RefCell<T>
    // allows immutable or mutable borrows checked at runtime.
    // 3. Because RefCell<T> allows mutable borrows checked at runtime,
    // you can mutate the value inside the RefCell<T>
    // even when the RefCell<T> is immutable.

    // Interior Mutability: A Mutable Borrow to an Immutable Value
    // let x = 5;
    // let y = &mut x;

    // move on to lib.rs

    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
