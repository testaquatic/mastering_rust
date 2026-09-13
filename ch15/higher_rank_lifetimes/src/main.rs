fn main() {
    let subtype: &for<'a> fn(&'a i32) -> &'a i32 = &((|x| x) as fn(&_) -> &_);
    let _supertype: &fn(&'static i32) -> &'static i32 = subtype;
}
