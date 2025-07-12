use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

struct Node<T: Debug + Display> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Debug + Display,
{
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }

    fn append(node: &Rc<RefCell<Node<T>>>, value: T) {
        let new_node = Node::new(value);
        let mut current_node = node.clone();
        while let Some(ref next_node) = current_node.clone().borrow().next {
            current_node = next_node.clone();
        }
        current_node.borrow_mut().next = Some(new_node);
    }
}

fn main() {
    let node = Node::new(1);
    Node::append(&node, 2);
    Node::append(&node, 3);

    let mut current = Some(Rc::clone(&node));
    while let Some(node) = current {
        print!("{} -> ", node.borrow().value);
        current = node.borrow().next.clone();
    }
    println!("None");
}
