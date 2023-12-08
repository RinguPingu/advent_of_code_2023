use core::panic;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    id: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(id: String, left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>) -> Self {
        Node { id, left, right }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    lines.next();

    let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new();

    for line in lines {
        let split = line
            .split('=')
            .map(|s| s.replace([' ', '(', ')'], ""))
            .collect::<Vec<String>>();
        let id: String = split.first().unwrap().to_string();

        let node = if let Some(x) = nodes.iter().find(|n| n.borrow().id == id) {
            x.clone()
        } else {
            nodes.push(Rc::new(RefCell::new(Node::new(id, None, None))));
            nodes.last().unwrap().clone()
        };

        let children = split.last().unwrap().split(',').collect::<Vec<&str>>();

        let left = if let Some(x) = nodes
            .iter()
            .find(|n| n.borrow().id == *children.first().unwrap())
        {
            x.clone()
        } else {
            nodes.push(Rc::new(RefCell::new(Node::new(
                children.first().unwrap().to_string(),
                None,
                None,
            ))));
            nodes.last().unwrap().clone()
        };
        let right = if let Some(x) = nodes
            .iter()
            .find(|n| n.borrow().id == *children.last().unwrap())
        {
            x.clone()
        } else {
            nodes.push(Rc::new(RefCell::new(Node::new(
                children.last().unwrap().to_string(),
                None,
                None,
            ))));
            nodes.last().unwrap().clone()
        };

        node.borrow_mut().left = Some(left);
        node.borrow_mut().right = Some(right);
    }

    println!("Finished processing nodes");

    let mut current_node = nodes
        .iter()
        .find(|n| n.borrow().id == "AAA")
        .unwrap()
        .clone();

    let mut steps: usize = 0;

    let mut it = instructions.chars().cycle();

    while current_node.borrow().id != "ZZZ" {
        println!("{}\t{}", steps, current_node.borrow().id);
        steps += 1;

        let next_node = match it.next().unwrap() {
            'L' => current_node.borrow().left.clone(),
            'R' => current_node.borrow().right.clone(),
            other => panic!("Instruction was {}", other),
        };

        current_node = next_node.unwrap();
    }

    println!("Reached ZZZ in {} steps!", steps);
}
