fn main() {
    println!("ch15-2!");
    // Treating Smart Pointers Like Regular References with the Deref Trait
    // Following the Pointer to the Value with the Dereference Operator
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    // Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Defining Our Own Smart Pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Treating a Type Like a Reference by Implementing the Deref Trait
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    *y;
    *(y.deref());

    // Implicit Deref Coercions with Functions and Methods
    // &String to &str

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    // How Deref Coercion Interacts with Mutability
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
}
