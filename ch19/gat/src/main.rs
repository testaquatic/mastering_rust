trait Borrower {
    type Item<'a>: 'a
    where
        Self: 'a;
    fn borrow<'a>(&'a self) -> Self::Item<'a>;
}

struct Book {
    title: String,
}

impl Borrower for Book {
    type Item<'a> = &'a str;

    fn borrow<'a>(&'a self) -> Self::Item<'a> {
        &self.title
    }
}

fn main() {
    let book = Book {
        title: "Rust Programming".to_string(),
    };
    let borrowed_title = book.borrow();
    println!("{}", borrowed_title);
}
