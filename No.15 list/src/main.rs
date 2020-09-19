fn main() {
    println!("Hello, world!");
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0}
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.toke()));
        self.head = Some(node);

    }
}


impl<T> Node<T> {
   pub fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
       Node {
           data: element,
           next,
       }
   }
}