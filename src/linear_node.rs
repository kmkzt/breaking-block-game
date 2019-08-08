pub enum LinearNode<T> {
    NonEmpty(Box<Node<T>>),
    Empty
}

pub struct Node<T> {
    pub node: T,
    pub next: LinearNode<T>
}

impl<T> LinearNode<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            LinearNode::Empty => 
                *self = LinearNode::NonEmpty(Box::new(Node {
                    node: value,
                    next: LinearNode::Empty
                })),
            LinearNode::NonEmpty(ref mut node) => 
                node.next.add(value)
        }
    }

    pub fn delete(&mut self) {
        match *self {
            LinearNode::NonEmpty(ref mut node) => 
                *self = node.next,
            _ => {},
        }
    }
}