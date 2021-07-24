enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    println!("ch15-4!");
    // Rc<T>, the Reference Counted Smart Pointer
    // fn first() {
    //     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //     let b = Cons(3, Box::new(a));
    //     let c = Cons(4, Box::new(a));
    // }
    // first();

    // Using Rc<T> to Share Data
    fn second() {
        use std::rc::Rc;
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }

    // Cloning an Rc<T> Increases the Reference Count
    fn third() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    third();
}
