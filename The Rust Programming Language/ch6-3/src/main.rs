fn main() {
    println!("ch6-3!");

    // Concise Control Flow with if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

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

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
