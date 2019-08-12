// use std::mem;
pub enum LinearNode<T> {
    NonEmpty(Box<Node<T>>),
    Empty
}

pub struct Node<T> {
    pub node: T,
    pub next: LinearNode<T>
}

impl<T> LinearNode<T> {
    pub fn new(value: Option<T>) -> Self {
        match value {
            Some(val) => LinearNode::NonEmpty(Box::new(Node {
                node: val,
                next: LinearNode::Empty
            })),
            _ => LinearNode::Empty
        }
    }
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

    
    pub fn update(&mut self, data: LinearNode<T>) {
        *self = data
    }

    // TODO: FIX E0507
    // pub fn delete(&mut self) {
    //     match self {
    //         LinearNode::Empty => *self = LinearNode::Empty,
    //         LinearNode::NonEmpty(node) => {
    //             match node.next() {
    //                 LinearNode::Empty => *self = LinearNode::Empty,
    //                 LinearNode::NonEmpty(next_node) => {
    //                     *self = mem::replace(self, &mut LinearNode::NonEmpty(next_node))
    //                 }
    //             } 
    //         },
    //     }
    // }
}

// impl<T> Node<T> {
//     pub fn next<'a>(&'a self) -> &'a LinearNode<T> {
//         &self.next
//     }
// }
