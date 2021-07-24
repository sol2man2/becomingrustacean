struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("ch15-3!");
    // Running Code on Cleanup with the Drop Trait
    fn first() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
    first();

    // Dropping a Value Early with std::mem::drop
    fn second() {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        // c.drop();
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
    second();
}
