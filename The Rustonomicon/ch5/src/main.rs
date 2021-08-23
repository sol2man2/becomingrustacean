fn first() {
    // let x: i32;
    // println!("{}", x);
}

fn second() {
    // let x: i32;

    // if true {
    //     x = 1;
    // } else {
    //     x = 2;
    // }

    // println!("{}", x);
}

// 모든 branch에서 초기화가 이루어져야 한다.

fn third() {
    // let x: i32;
    // if true {
    //     x = 1;
    // }
    // println!("{}", x);
}

fn fourth() {
    // let x: i32;

    // loop {
    //     // Rust doesn't understand that this branch will be taken unconditionally,
    //     // because it relies on actual values.
    //     if true {
    //         // But it does understand that it will only be taken once because
    //         // we unconditionally break out of it. Therefore `x` doesn't
    //         // need to be marked as mutable.
    //         x = 0;
    //         break;
    //     }
    // }
    // // It also knows that it's impossible to get here without reaching the break.
    // // And therefore that `x` must be initialized here!
    // println!("{}", x);
}

fn fifth() {
    // let x = 0;
    // let y = Box::new(0);
    // let z1 = x; // x is still valid because i32 is Copy
    // let z2 = y; // y is now logically uninitialized because Box isn't Copy
}

fn sixth() {
    // let mut y = Box::new(0);
    // let z = y; // y is now logically uninitialized because Box isn't Copy
    // y = Box::new(1); // reinitialize y
}

// variable이 init/deinit함에 따라 drop flag는 set/reset한다.
// initialize, deinitialize, reinitialize는 안전하게 이루어진다.__rust_force_expr!
// copy가 아닌 desturctor가 있으면 drop을 잘 봐야 한다.
fn seventh() {
    // let mut x = Box::new(0); // let makes a fresh variable, so never need to drop
    // let y = &mut x;
    // *y = Box::new(1); // Deref assumes the referent is initialized, so always drops
}

// 컴파일러는 static drop semantics을 가진다.
fn eighth() {
    // let mut x = Box::new(0); // x was uninit; just overwrite.
    // let mut y = x; // y was uninit; just overwrite and make x uninit.
    // x = Box::new(0); // x was uninit; just overwrite.
    // y = x; // y was init; Drop y, overwrite it, and make x uninit!
    //        // y goes out of scope; y was init; Drop y!
    //        // x goes out of scope; x was uninit; do nothing.
}

// branch에서 동일한 행동을 하게 되면 static drop semantics을 가짐
fn ninth() {
    // let mut x = Box::new(0); // x was uninit; just overwrite.
    //                          // if condition {
    // if true {
    //     drop(x) // x gets moved out; make x uninit.
    // } else {
    //     println!("{}", x);
    //     drop(x) // x gets moved out; make x uninit.
    // }
    // x = Box::new(0); // x was uninit; just overwrite.
    //                  // x goes out of scope; x was init; Drop x!
}

fn tenth() {
    // let x;
    // // if condition {
    // if true {
    //     x = Box::new(0); // x was uninit; just overwrite.
    //     println!("{}", x);
    // }
    // // x goes out of scope; x might be uninit;
    // // check the flag!
}

fn eleventh() {
    // // if condition {
    // if true {
    //     let x = Box::new(0);
    //     println!("{}", x);
    // }
}
// drop flag는 type에 붙지 않고 stack에 붙게 된다.

// 이 룰의 예외는 array이다.
// rust는 array를 한번에 초기화한다.
// let x = [val; N]
// let x = [val1, val2, val3]
fn twelfth() {
    // use std::mem::{self, MaybeUninit};

    // // Size of the array is hard-coded but easy to change (meaning, changing just
    // // the constant is sufficient). This means we can't use [a, b, c] syntax to
    // // initialize the array, though, as we would have to keep that in sync
    // // with `SIZE`!
    // const SIZE: usize = 10;

    // let x = {
    //     // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
    //     // safe because the type we are claiming to have initialized here is a
    //     // bunch of `MaybeUninit`s, which do not require initialization.
    //     let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };

    //     // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
    //     // assignment instead of `ptr::write` does not cause the old
    //     // uninitialized value to be dropped.
    //     // Exception safety is not a concern because Box can't panic
    //     for i in 0..SIZE {
    //         x[i] = MaybeUninit::new(Box::new(i as u32));
    //     }

    //     // Everything is initialized. Transmute the array to the
    //     // initialized type.
    //     unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
    // };

    // dbg!(x);
}

// uninit variable을 drop하는 꼴이다.
// 만일 MaybeUninit를 사용할 수 없다면 ptr module을 사용해라
// 아래 3함수는 old value를 drop하지 않고 할당하게 해 준다.
//  . ptr::write(ptr, val) takes a val and moves it into the address pointed to by ptr.
//  . ptr::copy(src, dest, count) copies the bits that count T's would occupy from src to dest. (this is equivalent to memmove -- note that the argument order is reversed!)
//  . ptr::copy_nonoverlapping(src, dest, count) does what copy does, but a little faster on the assumption that the two ranges of memory don't overlap. (this is equivalent to memcpy -- note that the argument order is reversed!)
// prt method를 사용하려고 raw pointer를 얻을 때 주의할 내용
//  . For an array of T, you can use base_ptr.add(idx) where base_ptr: *mut T to compute the address of array index idx. This relies on how arrays are laid out in memory.
//  . For a struct, however, in general we do not know how it is laid out, and we also cannot use &mut base_ptr.field as that would be creating a reference. Thus, it is currently not possible to create a raw pointer to a field of a partially initialized struct, and also not possible to initialize a single field of a partially initialized struct. (a solution to this problem is being worked on).
// deprecated mem::uninitialized는 초기화되지 않는 메모리를 다루는 방법이다.
fn thirteenth() {
    // *x[i].as_mut_ptr() = Box::new(i as u32); // WRONG!
}

fn main() {
    println!("ch5");

    first();
    second();
    third();
    fourth();
    fifth();
    sixth();
    seventh();
    eighth();
    ninth();
    tenth();
    eleventh();
    twelfth();
    thirteenth();
}
