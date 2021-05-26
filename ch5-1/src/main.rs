fn main() {
    println!("ch5-1!");

    // Defining and Instantiating Structs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Using the Field Init Shorthand when Variables and Fields Have the Same Name
    fn build_user1(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let user3 = User { ..user2 };

    // Using Tuple Structs without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    // ()와 같이 필드가 없는 struct로 10장에서 다룬다

    // Ownership of Struct Data
    // struct User1 {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }
}
