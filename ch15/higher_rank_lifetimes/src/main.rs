fn main() {
    let subtype: for<'a> fn(&'a i32) -> &'a i32 = (|x| x) as fn(&_) -> &_;
    #[allow(unused_variables)]
    let supertype: fn(&'static i32) -> &'static i32 = subtype;

    let subtype: &(dyn for<'a> Fn(&'a i32) -> &'a i32) = &|x| x;
    #[allow(unused_variables)]
    let supertype: &(dyn Fn(&'static i32) -> &'static i32) = subtype;

    let subtype: &for<'a, 'b> fn(&'a i32, &'b i32) = &((|_x, _y| {}) as fn(&_, &_));
    #[allow(unused_variables)]
    let supertype: &for<'c> fn(&'c i32, &'c i32) = subtype;
}
