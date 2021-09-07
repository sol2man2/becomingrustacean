#![feature(ptr_internals, allocator_api)]

use std::alloc::{Allocator, Global, GlobalAlloc, Layout};
use std::mem;
use std::ptr::{drop_in_place, NonNull, Unique};

struct Box<T> {
    ptr: Unique<T>,
    // not null인 ptr
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.ptr.as_ptr());
            let c: NonNull<T> = self.ptr.into();
            Global.deallocate(c.cast(), Layout::new::<T>())
        }
        // drop_in_place: pointer의 value를 drop하고 destructor를 실행
        // self.ptr.as_ptr(): *mut T를 반환
        // NonNull<T>: null이 아닌 값
        // self.ptr.into: non null을 반환
        // Global.deallocate: 메모리 반환
        // c.cast(): 해당 타입으로 변환
        // Layout::new::<T>(): 해당 타입으로 메모리 자리
    }
}

struct SuperBox<T> {
    my_box: Box<T>,
}

impl<T> Drop for SuperBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents
            let c: NonNull<T> = self.my_box.ptr.into();
            Global.deallocate(c.cast::<u8>(), Layout::new::<T>());
        }
        // drop_in_place를 실행하지 않아도 dangling 발생
    }
}

struct Boxy<T> {
    data1: Box<T>,
    data2: Box<T>,
    info: u32,
}
// data1, data2는 drop이 없더라도 drop된다.

enum Link {
    Next(Box<Link>),
    None,
}
// Link는 Next일 때만 drop이 발생한다.

struct SuperBox1<T> {
    my_box: Option<Box<T>>,
}

impl<T> Drop for SuperBox1<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents. Need to set the `box`
            // field as `None` to prevent Rust from trying to Drop it.
            let my_box = self.my_box.take().unwrap();
            let c: NonNull<T> = my_box.ptr.into();
            Global.deallocate(c.cast::<u8>(), Layout::new::<T>());
            mem::forget(my_box);
        }
        // self.my_box.take(): my_box에 None를 설정하고 가져온다.
        // mem::forget: 메모리를 관리 영역에서 잊어 버린다.
        // 항상 Some이여야 하는 field가 None될 수 있어서 이상하다.
    }
}

fn main() {
    println!("Ch6-2");
    // fn drop(&mut self);
    // 완전한 원자성의 소멸자이다.

    // drop 실행 후에, Rust는 자신의 모든 필드를 재귀적으로 drop할 것이다.
    // Rust 1.0에서 이런 행위를 막기위한 안정적인 방법은 없다.
    //  . &mut self는 억제해도 남아 있게 된다.
    //  . rust는 필드를 외부로 옮기는 것을 막을 것이다.
}
