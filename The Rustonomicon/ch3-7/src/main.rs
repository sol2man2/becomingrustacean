struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    F: Fn(&(u8, u16)) -> &u8,
    // F: Fn(&'??? (u8, u16)) -> &'??? u8,
    // for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}

fn do_it<'b>(data: &'b (u8, u16)) -> &'b u8 {
    &data.0
}

fn main() {
    println!("ch3-7 Higher-Rank Trait Bounds (HRTBs)");
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
}
