trait Trait<T> {
    fn do_something(&self, value: T);
}

struct TraitImpl;

impl Trait<&i32> for TraitImpl {
    fn do_something(&self, value: &i32) {
        println!("{}", value);
    }
}

fn foo(b: Box<dyn for<'a> Trait<&'a i32>>) {
    let x = 10;
    b.do_something(&x);
}

fn main() {
    foo(Box::new(TraitImpl));
}
