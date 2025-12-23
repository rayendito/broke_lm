use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    children: HashMap<String, usize>,
    value : Option<usize>
}

impl Trie {
    pub fn new() -> Self {
        let root = Node {
            children: HashMap::new(),
            value : None,
        };

        Trie {
            nodes: vec![root]
        }
    }

    pub fn insert(&mut self, key: &[String], value: usize) {
        let mut current = 0; // start at root
        for k in key { // need a way to also see if it's the last elemetn, if yes, set Value
            let next = {
                let node = &self.nodes[current]; // because we dont want to transfer ownership to this var.
                node.children.get(k).copied() // node.children is a hashmap, get() returns an Option (Some/None).
                // to be safe about the mutable borrows (because we're modifying nodes lateron)
                // we'll just return a copy here.
            };

            current = match next {
                Some(child_idx) => {
                    child_idx
                }
                None => {
                    let new_idx = self.nodes.len();

                    self.nodes.push(Node {
                        children: HashMap::new(),
                        value: None // we'll mark it's value last
                    });

                    self.nodes[current].children.insert(k.clone(), new_idx);

                    new_idx
                }
            }
        }
        self.nodes[current].value = Some(value);
    }

    pub fn debug_print(&self) {
        self.debug_print_recurse(0, 0);
    }

    fn debug_print_recurse(&self, index: usize, indent: usize){
        let node = &self.nodes[index];
        for (label, &child_idx) in node.children.iter() {
            for _ in 0..indent{
                print!("  ");
            }
            print!("{}", label);

            if let Some(v) = self.nodes[child_idx].value {
                print!("  [value = {}]", v);
            }

            println!();
            self.debug_print_recurse(child_idx, indent + 1);
        }

    }
}