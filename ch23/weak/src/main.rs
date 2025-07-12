use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

fn main() {
    let parent = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    });

    let child = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Rc::downgrade(&parent)),
        children: RefCell::new(Vec::new()),
    });

    parent.children.borrow_mut().push(Rc::clone(&child));

    let parent_weak = child.parent.borrow().upgrade();
    if let Some(parent_node) = parent_weak {
        println!("Child's parent value: {}", parent_node.value);
    } else {
        println!("Parent is no longer available.");
    }
}
