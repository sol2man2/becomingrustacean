trait Animal {
    fn snuggle(&self);
    fn eat(&mut self);
}

trait Cat: Animal {
    fn meow(&self);
}

trait Dog: Animal {
    fn bark(&self);
}

fn love(pet: Animal) {
    // pet.snuggle();
}

fn first() {
    // let mr_snuggles: Cat = ...;
    // love(mr_snuggles);         // ERROR: expected Animal, found Cat

    // cat도 animal이다
    // cat은 subtype, animal은 supertype
    // 간단한 규칙, supertype에 subtype이 대체될 수 있다.
}

fn evil_feeder(pet: &mut Animal) {
    // let spike: Dog = ...;

    // // `pet` is an Animal, and Dog is a subtype of Animal,
    // // so this should be fine, right..?
    // *pet = spike;

    // 그냥 대체 가능한가? 여기서 variance가 발생
}

fn second() {
    // let mut mr_snuggles: Cat = ...;
    // evil_feeder(&mut mr_snuggles);  // Replaces mr_snuggles with a Dog
    // mr_snuggles.meow();             // OH NO, MEOWING DOG!
}

fn third() {
    // big은 small을 포함
    // big은 small보다 오래 살고
    // big은 subtype, small은 supertype
    // 'static은 모든 lifetime의 subtype

    // variance
    // Vec는 T를 받아 Vec<T>를 반환하는 type constructor

    // F<T>에서 Sub, Super라는 type이 주어졌을 때
    // . F is covariant if F<Sub> is a subtype of F<Super> (subtyping "passes through")
    // . F is contravariant if F<Super> is a subtype of F<Sub> (subtyping is "inverted")
    // . F is invariant otherwise (no subtyping relationship exists)

    // F<T, U>인 경우는 T와 U에 대해서 각각 표현해 준다.

    // . Vec<T> and all other owning pointers and collections follow the same logic as Box<T>
    // . Cell<T> and all other interior mutability types follow the same logic as UnsafeCell<T>
    // . *const T follows the logic of &T
    // . *mut T follows the logic of &mut T (or UnsafeCell<T>)

    // references에서 lifetime
    // 1. short live가 필요한 곳에 long live를 넘긴다.
    // 2. lifetime은 reference의 일부이다. 두개 lifetime을 혼합할 방법이 없다.
}
fn evil_feeder1(pet: &mut Animal) {
    // let spike: Dog = ...;

    // // `pet` is an Animal, and Dog is a subtype of Animal,
    // // so this should be fine, right..?
    // *pet = spike;
}

fn fourth() {
    // let mut mr_snuggles: Cat = ...;
    // evil_feeder1(&mut mr_snuggles);  // Replaces mr_snuggles with a Dog
    // mr_snuggles.meow();             // OH NO, MEOWING DOG!
}

fn fifth() {
    // let mr_snuggles: Box<Cat> = ..;
    // let spike: Box<Dog> = ..;

    // let mut pet: Box<Animal>;
    // pet = mr_snuggles;
    // pet = spike;

    // 바로 덮어 쓰기 때문에 문제되지 않는다
}

fn sixth() {
    // fn get_animal() -> Animal {}
    // fn get_animal() -> Cat {}
    // fn handle_animal(Animal);
    // fn handle_animal(Cat);
}

use std::cell::Cell;
fn seventh() {
    // . If all uses of A are covariant, then MyType is covariant over A
    // . If all uses of A are contravariant, then MyType is contravariant over A
    // . Otherwise, MyType is invariant over A

    struct MyType<'a, 'b, A: 'a, B: 'b, C, D, E, F, G, H, In, Out, Mixed> {
        a: &'a A,     // covariant over 'a and A
        b: &'b mut B, // covariant over 'b and invariant over B

        c: *const C, // covariant over C
        d: *mut D,   // invariant over D

        e: E,       // covariant over E
        f: Vec<F>,  // covariant over F
        g: Cell<G>, // invariant over G

        h1: H,       // would also be variant over H except...
        h2: Cell<H>, // invariant over H, because invariance wins all conflicts

        i: fn(In) -> Out, // contravariant over In, covariant over Out

        k1: fn(Mixed) -> usize, // would be contravariant over Mixed except..
        k2: Mixed,              // invariant over Mixed, because invariance wins all conflicts
    }
}

fn main() {
    println!("ch3-8 Subtyping and Variance");

    first();
    second();
    third();
    fourth();
    fifth();
    sixth();
    seventh();
}
