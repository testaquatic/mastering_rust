struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn modify_first(&mut self, new_value: T) {
        if let Some(node) = self.head.as_mut() {
            node.value = new_value;
        }
    }
}

impl<T> LinkedList<T>
where
    T: std::fmt::Debug,
{
    fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = node.next.as_ref();
        }

        println!("None");
    }
}

fn main() {
    let mut list = LinkedList {
        head: Some(Box::new(Node {
            value: 1,
            next: None,
        })),
    };
    list.print();
    list.modify_first(42);
    list.print();
}
