struct ByteIter<'a> {
    remainder: &'a [u8],
}

impl<'a> ByteIter<'a> {
    fn next(&mut self) -> Option<&'a u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}

fn main() {
    let mut bytes = ByteIter { remainder: b"11234" };
    let byte1 = bytes.next();
    let byte2 = bytes.next();
    if byte1 == byte2 {
        println!("byte1 == byte2");
    }
}
