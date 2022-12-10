use rand::Rng;

pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: Node) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Node) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    fn search(&self, target: i32) -> Option<bool> {
        match self.value {
            value if target == value => Some(true),
            value if target < value => self.left.as_ref()?.search(target),
            value if target > value => self.right.as_ref()?.search(target),
            _ => Some(false),
        }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Some(Box::new(Node::new(value)));
        if value < self.value {
            match self.left.as_mut() {
                None => self.left = new_node,
                Some(left) => left.insert(value),
            }
        } else {
            match self.right.as_mut() {
                None => self.right = new_node,
                Some(right) => right.insert(value),
            }
        }
    }

    
}

fn main() {

    let mut rng = rand::thread_rng();

    println!("Insertando datos de prueba...");
    let mut root = Node::new(10);
    for _ in 0..4000 {
      root.insert(rng.gen_range(-40000..40000))
    }

    println!("Root: {}", root.value);
    println!("Search {}: {:?}", 12, root.search(12).unwrap_or_default());
}
