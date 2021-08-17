fn first() {
    let x;
    let y;
    // 변수는 역순으로 drop된다.
    // desugaring

    let tuple = (vec![], vec![]);
    // 왼쪽이 먼저 drop된다.
    // 오른쪽이 오래사는 것이 아니라 두 변수를 독립적으로 추적한다.
    // 주의하지 않으면 dangling point를 만들 수 있다.
}

fn second() {
    struct Inspector<'a>(&'a u8);

    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }
    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days));
}

fn third() {
    struct Inspector<'a>(&'a u8);

    impl<'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("I was only {} days from retirement!", self.0);
        }
    }

    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }

    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days));
    // Let's say `days` happens to get dropped first.
    // Then when Inspector is dropped, it will try to read free'd memory!
    // drop의 구현은 Inspector가 죽어 있는 동안 임의의 코드를 실행하는 꼴이다
}

fn fourth() {
    struct Inspector<'a>(&'a u8, &'static str);

    impl<'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }
    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }

    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days, "gadget"));
    // Let's say `days` happens to get dropped first.
    // Even when Inspector is dropped, its destructor will not access the
    // borrowed `days`.
}

fn fifth() {
    struct Inspector<T>(T, &'static str);

    impl<T> Drop for Inspector<T> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }

    struct World<T> {
        inspector: Option<Inspector<T>>,
        days: Box<u8>,
    }

    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days, "gadget"));
    // Let's say `days` happens to get dropped first.
    // Even when Inspector is dropped, its destructor will not access the
    // borrowed `days`.
}

// fourth, fifth 모두에서 borrow checker는 days가
// long live하지 않다고 할 것이다.
// borrow checker는 drop 내부에서 뭘 쓰느지 모르기 때문

fn sixth() {
    #![feature(dropck_eyepatch)]

    struct Inspector<'a>(&'a u8, &'static str);

    unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }

    struct World<'a> {
        days: Box<u8>,
        inspector: Option<Inspector<'a>>,
    }

    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days, "gatget"));
}

fn seventh() {
    #![feature(dropck_eyepatch)]
    use std::fmt::Display;

    struct Inspector<'a, 'b, T, U: Display>(&'a u8, &'b u8, T, U);

    unsafe impl<'a, #[may_dangle] 'b, #[may_dangle] T, U: Display> Drop for Inspector<'a, 'b, T, U> {
        fn drop(&mut self) {
            println!("Inspector({}, _, _, {})", self.0, self.3);
        }
    }
}

// eighth, ninth에서는 간접적으로 발생할 수 있다.
// invoking a callback,
// via a trait method call.

fn eighth() {
    struct Inspector<T>(T, &'static str, Box<for<'r> fn(&'r T) -> String>);

    impl<T> Drop for Inspector<T> {
        fn drop(&mut self) {
            // The `self.2` call could access a borrow e.g. if `T` is `&'a _`.
            println!(
                "Inspector({}, {}) unwittingly inspects expired data.",
                (self.2)(&self.0),
                self.1
            );
        }
    }
}

fn ninth() {
    use std::fmt;

    struct Inspector<T: fmt::Display>(T, &'static str);
    impl<T: fmt::Display> Drop for Inspector<T> {
        fn drop(&mut self) {
            // There is a hidden call to `<T as Display>::fmt` below, which
            // could access a borrow e.g. if `T` is `&'a _`
            println!(
                "Inspector({}, {}) unwittingly inspects expired data.",
                self.0, self.1
            );
        }
    }
}

fn main() {
    println!("ch3-9 Drop Check");
    first();
    second();
    third();
    fourth();
    fifth();
    sixth();
    seventh();
    eighth();
    ninth();
}
