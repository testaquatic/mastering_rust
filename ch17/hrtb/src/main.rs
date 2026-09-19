fn identify(s: &str) -> &str {
    s
}

fn example_fn<'a, 'b, F>(f: F)
where
    'a: 'b,
    F: Fn(&'a str) -> &'b str,
{
    let result = f("Hello, Rust!");
    println!("{}", result);
}

fn identify2<'a, 'b>(s: &'a str) -> &'b str
where
    'a: 'b,
{
    s
}

fn main() {
    example_fn(identify);
    example_fn(identify2);
}
