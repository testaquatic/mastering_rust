use std::fmt::Debug;

fn drop_static<T: 'static + Debug>(t: T) {
    dbg!("drop t: {:?}", &t);
    std::mem::drop(t);
}

fn main() {
    let strings = (0..10)
        .into_iter()
        .map(|_| rand::random::<u64>().to_string())
        .collect::<Vec<_>>();

    for mut string in strings {
        string.push_str("a mutation");
        drop_static(string);
    }

    println!("I am and of program");
}
