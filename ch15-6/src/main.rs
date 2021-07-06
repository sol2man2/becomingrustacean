use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    println!("ch15-6!");
    // Reference Cycles Can Leak Memory
    fn first() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    // Creating a Tree Data Structure: a Node with Child Nodes
    fn second() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }

    // Adding a Reference from a Child to Its Parent
    // fn third() {
    //     let leaf = Rc::new(Node {
    //         value: 3,
    //         parent: RefCell::new(Weak::new()),
    //         children: RefCell::new(vec![]),
    //     });
    //     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //     let branch = Rc::new(Node {
    //         value: 5,
    //         parent: RefCell::new(Weak::new()),
    //         children: RefCell::new(vec![Rc::clone(&leaf)]),
    //     });
    //     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    //     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // }

    // Visualizing Changes to strong_count and weak_count
    // fn fourth() {
    //     let leaf = Rc::new(Node {
    //         value: 3,
    //         parent: RefCell::new(Weak::new()),
    //         children: RefCell::new(vec![]),
    //     });
    //     println!(
    //         "leaf strong = {}, weak = {}",
    //         Rc::strong_count(&leaf),
    //         Rc::weak_count(&leaf),
    //     );
    //     {
    //         let branch = Rc::new(Node {
    //             value: 5,
    //             parent: RefCell::new(Weak::new()),
    //             children: RefCell::new(vec![Rc::clone(&leaf)]),
    //         });
    //         *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    //         println!(
    //             "branch strong = {}, weak = {}",
    //             Rc::strong_count(&branch),
    //             Rc::weak_count(&branch),
    //         );
    //         println!(
    //             "leaf strong = {}, weak = {}",
    //             Rc::strong_count(&leaf),
    //             Rc::weak_count(&leaf),
    //         );
    //     }
    //     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //     println!(
    //         "leaf strong = {}, weak = {}",
    //         Rc::strong_count(&leaf),
    //         Rc::weak_count(&leaf),
    //     );
    // }

    // first();
    // second();
    // third();
    // fourth();
}

#[derive(Debug)]
struct Node {
    value: i32,
    // parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
