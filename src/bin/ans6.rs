use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::io;

struct Node {
    is_root: bool,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Node { is_root: true, children: Vec::new() }
    }
}

struct World {
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl World {
    fn new() -> Self {
        World { nodes: HashMap::new() }
    }

    fn add_edge(&mut self, parent: &str, child: &str) {
        let parent_node = self.nodes.entry(parent.into()).or_default().clone();
        let child_node = self.nodes.entry(child.into()).or_default().clone();
        child_node.borrow_mut().is_root = false;
        parent_node.borrow_mut().children.push(child_node);
    }

    fn add_edge_from_string(&mut self, line: &str) -> Option<()> {
        let (parent, child) = line.trim_end().split_once(")")?;
        self.add_edge(parent, child);
        Some(())
    }

    fn find_all_roots(&self) -> Vec<Rc<RefCell<Node>>> {
        self.nodes.values()
            .filter(|node| node.borrow().is_root)
            .map(|x| x.clone())
            .collect()
    }

    fn _compute_orbits_recursive(&self, node: Rc<RefCell<Node>>, depth: i32) -> i32 {
        depth +
            node.borrow().children.iter()
            .map(|n| self._compute_orbits_recursive(n.clone(), depth+1)).sum::<i32>()
    }

    fn compute_total_orbits(&self) -> i32 {
        self.find_all_roots().iter().map(|node| self._compute_orbits_recursive(node.clone(), 0)).sum::<i32>()
    }

    // the result path is reversed (to -> from)
    fn _find_path_recursive(&self, from: Rc<RefCell<Node>>, to: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        if Rc::ptr_eq(&from, &to) {
            return vec![from];
        }
        for next_node in &from.borrow().children {
            let mut next_path = self._find_path_recursive(next_node.clone(), to.clone());
            if !next_path.is_empty() {
                next_path.push(from.clone());
                return next_path;
            }
        }
        return Vec::new()
    }

    fn compute_distance(&self, src: &str, dst: &str) -> i32 {
        let root = self.nodes.get("COM").unwrap();
        let src_node = self.nodes.get(src).unwrap();
        let dst_node = self.nodes.get(dst).unwrap();

        let mut root2src = self._find_path_recursive(root.clone(), src_node.clone());
        let mut root2dst = self._find_path_recursive(root.clone(), dst_node.clone());

        while !root2src.is_empty() && !root2dst.is_empty()
            && Rc::ptr_eq(root2src.last().unwrap(), root2dst.last().unwrap()) {
                root2src.pop();
                root2dst.pop();
            }
        (root2src.len() + root2dst.len() - 2) as i32
    }
}

#[test]
fn test_0() {
    let mut world = World::new();
    world.add_edge_from_string("COM)B");
    world.add_edge_from_string("B)C");
    world.add_edge_from_string("C)D");
    world.add_edge_from_string("D)E");
    world.add_edge_from_string("E)F");
    world.add_edge_from_string("B)G");
    world.add_edge_from_string("G)H");
    world.add_edge_from_string("D)I");
    world.add_edge_from_string("E)J");
    world.add_edge_from_string("J)K");
    world.add_edge_from_string("K)L");
    assert_eq!(world.compute_total_orbits(), 42);

    world.add_edge_from_string("K)YOU");
    world.add_edge_from_string("I)SAN");
    assert_eq!(world.compute_distance("YOU", "SAN"), 4);
}

fn main() {
    let mut world = World::new();
    for line in io::stdin().lines() {
        world.add_edge_from_string(&line.unwrap());
    }
    println!("total orbits: {}", world.compute_total_orbits());
    println!("distance YOU to SAN: {}", world.compute_distance("YOU", "SAN"));
}
