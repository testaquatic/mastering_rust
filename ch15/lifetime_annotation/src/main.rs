struct ByteIter<'remainder> {
    remainder: &'remainder [u8],
}

impl<'remainder> ByteIter<'remainder> {
    fn next(&mut self) -> Option<&'remainder u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}

#[derive(Clone, Debug)]
#[allow(unused)]
struct NumberRef<'a>(&'a i32);

impl<'a> NumberRef<'a> {
    fn some_method(&mut self) {}
}

fn main() {
    let mut bytes: ByteIter<'static> = ByteIter {
        remainder: b"hello",
    };
    let byte_1 = bytes.next();
    let byte_2 = bytes.next();

    if byte_1 == byte_2 {
        println!("equal");
    } else {
        println!("not equal");
    }

    let mut num_ref = NumberRef(&5);
    num_ref.some_method();
    num_ref.some_method();

    println!("{:?}", num_ref);
}
