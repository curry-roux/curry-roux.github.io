use std::collections::HashMap;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

type NodeRef = Rc<Node>;

#[derive(Clone, Hash, Eq, PartialEq)]
struct NodeKey(Option<NodeRef>, Option<NodeRef>, Option<NodeRef>, Option<NodeRef>);

struct Node {
    level: u32,
    population: u32,
    nw: NodeRef,
    ne: NodeRef,
    sw: NodeRef,
    se: NodeRef,
    next_cache: std::cell::RefCell<Option<NodeRef>>,
}

impl Node {
    fn new(level: u32, nw: NodeRef, ne: NodeRef, sw: NodeRef, se: NodeRef) -> NodeRef {
        let population = nw.population + ne.population + sw.population + se.population;
        Rc::new(Node {
            level,
            population,
            nw,
            ne,
            sw,
            se,
            next_cache: std::cell::RefCell::new(None),
        })
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level &&
        self.population == other.population &&
        Rc::ptr_eq(&self.nw, &other.nw) &&
        Rc::ptr_eq(&self.ne, &other.ne) &&
        Rc::ptr_eq(&self.sw, &other.sw) &&
        Rc::ptr_eq(&self.se, &other.se)
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.level.hash(state);
        self.population.hash(state);
        // hash 子ノードはRcのポインタアドレスで判定
        (Rc::as_ptr(&self.nw) as usize).hash(state);
        (Rc::as_ptr(&self.ne) as usize).hash(state);
        (Rc::as_ptr(&self.sw) as usize).hash(state);
        (Rc::as_ptr(&self.se) as usize).hash(state);
        // next_cacheは無視する
    }
}

struct HashLife {
    cache: HashMap<NodeKey, NodeRef>,
    leaf0: NodeRef,
    leaf1: NodeRef,
}

impl HashLife {
    fn new() -> Self {
        let leaf0 = Rc::new(Node {
            level: 0,
            population: 0,
            nw: Rc::new_cyclic(|_| unreachable!()),
            ne: Rc::new_cyclic(|_| unreachable!()),
            sw: Rc::new_cyclic(|_| unreachable!()),
            se: Rc::new_cyclic(|_| unreachable!()),
            next_cache: std::cell::RefCell::new(None),
        });
        let leaf1 = Rc::new(Node {
            level: 0,
            population: 1,
            nw: Rc::new_cyclic(|_| unreachable!()),
            ne: Rc::new_cyclic(|_| unreachable!()),
            sw: Rc::new_cyclic(|_| unreachable!()),
            se: Rc::new_cyclic(|_| unreachable!()),
            next_cache: std::cell::RefCell::new(None),
        });
        let mut hl = HashLife { cache: HashMap::new(), leaf0, leaf1 };
        hl.cache.insert(NodeKey(None, None, None, None), hl.leaf0.clone());
        hl.cache.insert(NodeKey(Some(hl.leaf1.clone()), None, None, None), hl.leaf1.clone());
        hl
    }

    fn get_node(&mut self, level: u32, nw: NodeRef, ne: NodeRef, sw: NodeRef, se: NodeRef) -> NodeRef {
        let key = NodeKey(Some(nw.clone()), Some(ne.clone()), Some(sw.clone()), Some(se.clone()));
        if let Some(node) = self.cache.get(&key) {
            return node.clone();
        }
        let node = Node::new(level, nw, ne, sw, se);
        self.cache.insert(key, node.clone());
        node
    }

    fn expand_universe(&mut self, node: NodeRef) -> NodeRef {
        let empty = self.zero_tree(node.level - 1);
        let nw = self.get_node(node.level, empty.clone(), empty.clone(), empty.clone(), node.nw.clone());
        let ne = self.get_node(node.level, empty.clone(), empty.clone(), node.ne.clone(), empty.clone());
        let sw = self.get_node(node.level, empty.clone(), node.sw.clone(), empty.clone(), empty.clone());
        let se = self.get_node(node.level, node.se.clone(), empty.clone(), empty.clone(), empty.clone());

        self.get_node(
            node.level + 1,
            nw,
            ne,
            sw,
            se,
        )
    }

    fn zero_tree(&mut self, level: u32) -> NodeRef {
        if level == 0 {
            return self.leaf0.clone();
        }
        let z = self.zero_tree(level - 1);
        self.get_node(level, z.clone(), z.clone(), z.clone(), z)
    }

    fn next_generation(&mut self, node: NodeRef) -> NodeRef {
        if node.population == 0 {
            return self.zero_tree(node.level - 1);
        }
        if let Some(cached) = node.next_cache.borrow().clone() {
            return cached;
        }
        let result = if node.level == 1 {
            // base case: 2x2 block -> next 1x1
            let sum = node.nw.population + node.ne.population + node.sw.population + node.se.population;
            let alive = (sum == 3) || (sum == 2 && node.ne.population == 1);
            if alive { self.leaf1.clone() } else { self.leaf0.clone() }
        } else {
            // recursive case
            let center = self.get_center(node.nw.clone(), node.ne.clone(), node.sw.clone(), node.se.clone());
            let n00 = self.next_generation(center);
            n00
        };
        node.next_cache.replace(Some(result.clone()));
        result
    }

    fn get_center(&mut self, nw: NodeRef, ne: NodeRef, sw: NodeRef, se: NodeRef) -> NodeRef {
        let a = self.get_node(nw.level - 1, nw.nw.clone(), nw.ne.clone(), nw.sw.clone(), nw.se.clone());
        a
    }
}

fn main() {
    let mut hl = HashLife::new();
    // Example: single cell
    let cell = hl.get_node(0, hl.leaf1.clone(), hl.leaf0.clone(), hl.leaf0.clone(), hl.leaf0.clone());
    let root = hl.get_node(1, cell.clone(), hl.leaf0.clone(), hl.leaf0.clone(), hl.leaf0.clone());
    let next = hl.next_generation(root.clone());
    println!("Next pop: {}", next.population);
}
