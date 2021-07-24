fn main() {
    println!("ch3-2!");

    // Data Types
    let guess: u32 = "41".parse().expect("not a number");
    // Scalar Types
    // Integer Types
    let x: u32 = 0;
    // i8, i16, i32, i64, i128, isize,
    // u8, u16, u32, u64, u128, usize
    let mut y: u8 = 255;
    // y = y + 1;

    let mut z: u8 = "255".parse().expect("whoops!");
    // z = z + 1;
    println!("z {}", z);

    // Floating-Point Types
    let a = 2.0; // f64
    let b: f32 = 3.0; //f32

    // Numeric Operations
    let c = 2 + 3;
    let d = 389.2 - 32.3;
    let e = 4 * 30;
    let f = 26 / 3;
    let g = 43 % 5;

    println!("c,d,e,f,g: {}, {}, {}, {}, {}", c, d, e, f, g);

    // The Boolean Type
    let bt = true;
    let bf: bool = false;

    // The Character Type
    let c1 = 'z';
    let c2 = 'ℤ';
    let c3 = '😻';
    println!("{} {} {}", c1, c2, c3);

    // Compound Types
    // The Tuple Type
    // let tup: (i32, f64, u8) = (300, 6.2, 1);
    let tup = (300, 6.2, 1);
    let (x1, x2, x3) = tup;
    println!("xs: {} {} {}", x1, x2, x3);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // The Array Type
    let arr1 = [1, 2, 3, 4, 5];

    let arr2: [i32; 3] = [1, 2, 3];
    let arr3 = [4; 6];
    println!("arr2, {:?}", arr2);
    println!("arr3, {:?}", arr3);

    // Accessing Array Elements
    println!("arr1, {} {}", arr1[0], arr1[3]);

    // Invalid Array Element Access
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
    let index: usize = index.trim().parse().expect("");
    println!("invalid: {}", arr1[index]);
}
