struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: i32,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            left: None,
            right: None,
            data,
        }
    }
    fn max_depth(&self) -> u32 {
        if self.left.is_none() && self.right.is_none() {
            return 1; // Depth of a leaf node is 1, not 0
        }
        let left_depth = match &self.left {
            Some(node) => 1 + node.max_depth(),
            None => 0,
        };
        let right_depth = match &self.right {
            Some(node) => 1 + node.max_depth(),
            None => 0,
        };
        left_depth.max(right_depth)
    }
}

pub fn run() {
    //Here the node of a binary tree can be passed as an argument to the max_depth function.
    //This is just a small simulation of how the function can be used.
    let mut root = Node::new(1);
    let left = Node::new(2);
    let right = Node::new(3);
    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    println!("Maximum Depth: {}", root.max_depth());
}
