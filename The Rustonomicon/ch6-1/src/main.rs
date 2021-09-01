fn main() {
    println!("Ch6-1");
    // OBRM, RAII
    // 메모리뿐 아니라 다른 자원에도 적용된다.
    struct Foo {
        a: u8,
        b: u32,
        c: bool,
    }
    // impl Foo {
    //     fn new(a: u8, b: u32, c: bool) -> Foo {
    //         Foo { a, b, c }
    //     }
    // }

    enum Bar {
        X(u32),
        Y(bool),
    }
    struct Unit;

    let foo = Foo {
        a: 0,
        b: 1,
        c: false,
    };
    let bar = Bar::X(0);
    let empty = Unit;
    // c++과 달리 내장 생성자가 많지 않다
    // rust는 move만 있어서 그렇다
}
