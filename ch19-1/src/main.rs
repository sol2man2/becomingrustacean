fn main() {
    println!("ch19-1!");

    // Unsafe Rust
    // Unsafe Superpowers
    // . Dereference a raw pointer
    // . Call an unsafe function or method
    // . Access or modify a mutable static variable
    // . Implement an unsafe trait
    // . Access fields of unions

    // Dereferencing a Raw Pointer

    // Different from references and smart pointers, raw pointers:
    // . Are allowed to ignore the borrowing rules
    //   by having both immutable and mutable pointers
    //   or multiple mutable pointers to the same location
    // . Aren’t guaranteed to point to valid memory
    // . Are allowed to be null
    // . Don’t implement any automatic cleanup
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r);
    }
    // why raw pointer?
    // c 언어와 인터페이스
    // 대여 검사기가 이해할 수 없는 추상화 만들기

    // Calling an Unsafe Function or Method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    // dangerous();
    // dangerous()를 unsafe에 넣으면서
    // dangerous()에 대한 문서를 충분히 읽고 적절히 사용한다.

    // Creating a Safe Abstraction over Unsafe Code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     assert!(mid <= len);
    //     (&mut slice[..mid], &mut slice[mid..])
    // }

    use std::slice;
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // slice를 사용할 때 오류를 냄

    // Using extern Functions to Call External Code
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Calling Rust Functions from Other Languages
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // Accessing or Modifying a Mutable Static Variable
    println!("name is: {}", HELLO_WORLD);

    // Constants: 여러곳에 저장 됨
    // immutable static variables: 하나의 위치에 저장 후 참조

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Implementing an Unsafe Trait
    unsafe trait Foo {
        // methods go here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
