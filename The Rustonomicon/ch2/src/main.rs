fn main() {
    println!("Hello, world!");
}

/*
ch2

////////////
Data Represendtation in rust

////////////
repr(rust)
rust는 다음으로 데이터 레이아웃을 준다.
* structs (named product types)
* tuples (anonymous product types)
* arrays (homogeneous product types)
* enums (named sum types -- tagged unions)
* unions (untagged unions)

struct A {
    x: i8,
    y: i8,
}
product types

enum B {
    Person,
    Animal,
}
sum types

Option -> None, Some<T>

composite structure는 필드 alignment의 최대값과 동일하며 패딩을 한다.
struct A {
    a: u8,
    b: u32,
    c: u16,
}

struct A {
    a: u8,
    _pad1: [u8; 3], // to align `b`
    b: u32,
    c: u16,
    _pad2: [u8; 2], // to make overall size multiple of 4
}
또는
struct A {
    b: u32,
    c: u16,
    a: u8,
    _pad: u8,
}

아래 두 정의에서
struct A {
    a: i32,
    b: u64,
}
struct B {
    a: i32,
    b: u64,
}
두개의 A는 레이아웃이 동일하지만
A, B간에는 보장할 수 없다.

A, B를 쓸때, 너무 세세하게 규칙을 잡는 것 같지만
rust feature로 보면 복잡하게 작동하길 바란다.
struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U,
}

Foo<u32, u16> 와 Foo<u16, u32>를 고려해 보면,
struct Foo<u16, u32> {
    count: u16,
    data1: u16,
    data2: u32,
}
struct Foo<u32, u16> {
    count: u16,
    _pad1: u16,
    data1: u32,
    data2: u16,
    _pad2: u16,
}

enum은 이런 고민을 더 복잡하게 만든다.
enum Foo {
    A(u32),
    B(u64),
    C(u8),
}
는
struct FooRepr {
    data: u64, // this is either a u64, u32, or u8 based on `tag`
    tag:u8, //0=A,1=B,2=C
}

이런 관리의 고전적 처리는
null, not-null로 처리하면 tag가 쓸모없게 된다.
이경우 size_of::<Option<&T>>() == size_of::<&T>()

////////////
Exotically sized types

Dynamically sized type(DSTs)
러스트가 지원하는 정적이지 않고 동적인 타입니다.
러스트는 사이즈를 알아야 하지만 이것은 특별한 타입이다

언어가 주는 DSTs의 일반적인 두 타입
* trait objects: dyn MyTrait
* slices: [T] , str , and others

struct MySuperSlice {
    info: u32,
    data: [u8],
}
struct는 single DST를 스택에 저장하는 형태이다.

struct MySuperSlicestruct LotsOfNothing {
    foo: Nothing,
    qux: (),      // empty tuple has no size
    baz: [u8; 0], // empty array has no size
}

fn main() {
    let sized: MySuperSliceable<[u8; 8]> = MySuperSliceable {
        info: 17,
        data: [0; 8],
    };
    let dynamic: &MySuperSliceable<[u8]> = &sized;
    println!("{} {:?}", dynamic.info, &dynamic.data);
    // prints: "17 [0, 0, 0, 0, 0, 0, 0, 0]"
}

Zero Sized Types (ZST)

struct Nothing; // No fields = no size

// All fields have no size = no size
struct LotsOfNothing {
    foo: Nothing,
    qux: (),      // empty tuple has no size
    baz: [u8; 0], // empty array has no size
}
zst는 쓸모없지만 잠재성은 generic context에서 발현된다.
zst를 만들고 저장하는 동작은 no-op으로 줄어든다.
극단적인 예로 Map<Key, Value>가 주어졌을 때
Map<Key, UselessJunk>처럼 Set<Key>를 구현하는 것이 일반적이다.
rust에서는 Set<Key> = Map<Key, ()>라고 말한다.
HashMap의 오버해드가 없는 HashSet이다.

////////////
Empty Types
러스트는 인스턴스화 할 수 없는 타입도 선언한다.
타입 레벨에서만 얘기 가능하고 밸류 레벨에서는 논의 불가

enum Void {} // No variants = EMPTY
empty types은 zst보다 더 극단적이다.
Result<T, Void>로 표현가능하다.
Result<T, Void>는 T이다.

enum Void {}
let res: Result<u32, Void> = Ok(0);
// Err doesn't exist anymore, so Ok is actually irrefutable.
let Ok(num) = res;
C에서 void*는  *const Void 이다.

////////////
Extern Types
알려지지 않은 사이즈로 타입을 정의하는 걸 extern types라 한다.

////////////
Alternative represenatations

////////////
repr(c)는 c가 하는 것들이다. c,  c++에서 기대하는 전부를 가진다.
repr(c)는 lingua franca 같은 것이다.

rust-bindgen, cbindgen crates를 사용하길 강력히 권한다.

////////////
repr(transparent)
single non-zero sized field

struct Foo(f32)는 f32와 같다

////////////
repr(u*), repr(i*)
fieldless enum이라고 하고

enum MyOption<T> {
    Some(T),
    None,
}

#[repr(u8)]
enum MyReprOption<T> {
    Some(T),
    None,
}
assert_eq!(8, size_of::<MyOption<&u16>>());
assert_eq!(16, size_of::<MyReprOption<&u16>>());

////////////
repr(packed)
repr(packed)는 러스트가 패딩을 제거하고 타입을 바이트로 가지런히 한다.
메모리 자취를 향상하지만 부정적 사아트 이펙트가 있다.

////////////
repr(align(n))
n은 2의 제곱수이고 타입이 적어도 n의 할당을 가지게 한다.

*/
