fn main() {
    println!("Hello, world!");
}

/*
ch1
meet safe and unsafe

상세 구현이 중요하다고 생각될 때 프로그래머는 3개의 옵션을 생각
1. 컴파일러와 런타임이 최적화 하게 코드를 만진다.
2. 원하는 구현을 위해 관용적이거나 거대한 설계를 적용
3. 세부 내용을 다룰수 있는 언어로 재작성 (C???)

C는 비안정적이라서 함께 쓰면 안전성 떨어진다.
* c와 달리 rust는 안전한 언어다.
* c처럼 rust는 불안전한 언어다.
* 두가지 특성 모두를 가진다

safe rust에서 프로그램하면 행복하지만 unsafe rust를 쓰게 될 것이다.
unsafe rust는 저수준에서 상세 구현이 가능하게 해 준다.

////////////
How safe and unsae interact
다른 코드와 함께 할 때 unsafe keyword가 사용된다.
#![forbid(unsafe_code)]로 막을 수 있다.

unsafe 의 두가지 사용
• 컴파일러가 확인할 수 없는 계약을 선언할 때
  func, trait에 이런 계약이 있다고 알려 준다
  function: f사용자가 문서를 잘 봐야 한다.
  trait: t사용자가 문서를 잘 봐서 구현해라.
• 프로그래머가 이 계약을 유지한다고 선언할 때

표준 라이브러리가 가지는 unsafe
• slice::get_unchecked내부에서 unchecked index를 사용한다.
• mem::transmute
• sized type에 대한 모든 raw pointer는 offset 메소드를 가진다.
  (이게 undefined동작 수행가능)
• 모든 FFI는 unsafe 하다.

let raw_bytes = [0x78, 0x56, 0x34, 0x12];
let num = unsafe {
    std::mem::transmute::<[u8; 4], u32>(raw_bytes)
};
let num = u32::from_ne_bytes(raw_bytes);
let num = u32::from_le_bytes(raw_bytes);
assert_eq!(num, 0x12345678);
let num = u32::from_be_bytes(raw_bytes);
assert_eq!(num, 0x78563412);

다음의 unsafe trait을 정의한다.
• Send: 구현자가 다른 스레드에 send하는게 안전하다를 약속한다.
• Sync: 공유된 레퍼런스를 통해서 구현자를 공유할 수 있는 약속
• GlobalAlloc: 전체 프로그램에 메모리 할당을 허락

많은 rust std. lib.들은 내부적으로 unsafe를 사용한다.

////////////
safe rust는 unsafe 활동을 할 수 없다.
safe rust는 unsafe rust를 신뢰한다. 반대는 주의해야 한다.

PartialOrd, Ord
BTreeMap은 부분 순위 타입에는 적절하지 않다. Ord를 필요로 한다.
BTreeMap은 내부적으로 unsafe를 가지고 있고 safe인 Ord에 대해 robust하게 구현되어야 한다.
unsafe rust는 safe rust를 신뢰할 수 없다.
BTreeMap에서 int와 slice가 사용되는데 이걸 신뢰하는데 이 신뢰가 깨지면 모든게 깨진다. 그래서 BTreeMap를 유지보수하는 사람이 꾸준이 봐 줘야 한다.
BTreeMap 유지보수자는 generic에서 ord가 깨지지 않게 봐줘야 한다.
BTreeMap 과거, 현재, 미래에 safe하다고 확신 불가
이런 unbounded generic trust문제 때문에 UnsaeOrd가 만들어졌다.
BTreeMap은 Ord 보다는 UnsafeOrd로 구현하기로 되어 있다.

use std::cmp::Ordering;
unsafe trait UnsafeOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}

trait가 기대하는 safe를 위해서 유지 보수해야 한다.
unsafe trait가 안전한지 확인하게 유도한다.
unsafe trait가 많아지는 것을 우려해서 rust는 많은 사용을 권장하지 않는다
safe trait는 구현하기 쉽다. unsafe trait는 어려워서 구현자에게 책임을 넘기는 것이다.

Send, Sync는 자동 구현이다. Send, Sync는 Send, Sync로 구현된 타입의 값으로 구성되어 있을 때 자동으로 상속되어 구현된다.

////////////
What unsafe rust can do
unsafe rust에서 safe rust와 차이 나는 것들은
• Dereference raw pointers
• Call functions (including C functions, compiler intrinsics, and the raw allocator)
• Implement unsafe traits
• Mutate statics
• Access fields of union s

사용자에게 미정의 행위를 할 수 있게 열어 준다.
C와 달리 미정의 행위가 아주 제한적이다.

포인터가 널이거나 부분 메모리를 가리키면 댕글링 포인터가 된다.
slice, string에 포인터나 사이즈가 잘못되는 경우 등등
offset이 isize::MAX를 넘을 땐 raw pointer를 써야 한다.

Vec, Box는 자신드의 포인터를 항상 non null로 유지하기 위해서 intrinsic(고유성)을 사용한다.

이런 상황에 안전하게 고려한다.
* Deadlock
* Have a race condition
* Leak memory
* Fail to call destructors
* Overflow integers
* Abort the program
* Delete the production database

////////////
working with unsafe

fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx < arr.len() {
        // return Option(arr[idx]);
        unsafe { Some(*arr.get_unchecked(idx))}
    } else {
        None
    }
}
이 코드는 안전하고 바르다. 우리는 인텍스를 확인한다.
올바른 unsafe 구현이다. 즉, safe code가 미정의 행위를 야기할 수 없다는 의미

< -> <=로 수정하면 범위가 바뀌게 된다.
이 프로그램은 절적하지 않다. safe code가 미정의 행위를 야기할 수 있다.
safe code를 수정했을 뿐이다.

use std::ptr;
// Note: This definition is naive. See the chapter on implementing Vec.
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note this implementation does not correctly handle zero-sized types. // See the chapter on implementing Vec.
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // not important for this example
            self.reallocate();
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), elem); self.len += 1;
        }
    }
}
전체적으로 문제없다.

fn make_room(&mut self) {
    // grow the capacity
    self.cap += 1;
}
이 코드는 safe rust지만 완전 문제다.
*/
