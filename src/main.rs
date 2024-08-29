struct Node {
  dato: i32,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

impl Node {
  fn new(value: i32) -> Node {
    Node {
      dato: value,
      left: None,
      right: None,
    }
  }

  fn set_left(&mut self, node: &mut Node) -> () {}
  fn set_right(&mut self, node: &mut Node) -> () {}
}

fn inorder(node: &Option<Box<Node>>) {
  if let Some(inner_node) = node {
    inorder(&inner_node.left);
    println!("Dato: {}", inner_node.dato);
    inorder(&inner_node.right);
  }
}

fn preorder(node: &Option<Box<Node>>) {
  if let Some(inner_node) = node {
    println!("Dato: {}", inner_node.dato);
    preorder(&inner_node.left);
    preorder(&inner_node.right);
  }
}

fn postorder(node: &Option<Box<Node>>) {
  if let Some(inner_node) = node {
    postorder(&inner_node.left);
    postorder(&inner_node.right);
    println!("Dato: {}", inner_node.dato);
  }
}

fn factorial(x: i32) -> i32 {
  if x == 1 {
    return 1;
  }
  x * factorial(x - 1)
}

fn main() {
  let mut root = Node::new(6);
  let mut node1 = Node::new(4);
  let mut node2 = Node::new(1);
  let mut node3 = Node::new(5);
  let mut node4 = Node::new(8);
  let mut node5 = Node::new(9);

  // root.set_left(&mut node1);
  // root.set_right(&mut node4);

  // node1.set_left(&mut node2);
  // node1.set_left(&mut node3);

  // node4.set_right(&mut node5);

  // postorder(&Some(Box::new(root)));
}
