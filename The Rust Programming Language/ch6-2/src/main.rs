fn main() {
    println!("ch6-2!");

    // The match Control Flow Operator
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }
    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => {
    //             println!("Lucky penny!");
    //             1
    //         }
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    // Patterns that Bind to Values
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matches Are Exhaustive
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // The _ Placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
