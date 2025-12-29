use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::train_utils::tokenize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trie {
    nodes: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    children: HashMap<String, usize>,
    fail: usize,
    value: Option<usize>,
}

impl Trie {
    pub fn new() -> Self {
        let root = Node {
            children: HashMap::new(),
            fail: 0,
            value: None,
        };

        Trie { nodes: vec![root] }
    }

    pub fn insert(&mut self, key: &[String], value: usize) {
        let mut current = 0; // start at root
        for k in key {
            // need a way to also see if it's the last elemetn, if yes, set Value
            let next = {
                let node = &self.nodes[current]; // because we dont want to transfer ownership to this var.
                node.children.get(k).copied() // node.children is a hashmap, get() returns an Option (Some/None).
                // to be safe about the mutable borrows (because we're modifying nodes lateron)
                // we'll just return a copy here.
            };

            current = match next {
                Some(child_idx) => child_idx,
                None => {
                    let new_idx = self.nodes.len();

                    self.nodes.push(Node {
                        children: HashMap::new(),
                        fail: 0,     // we'll update these during inference
                        value: None, // we'll mark it's value last
                    });

                    self.nodes[current].children.insert(k.clone(), new_idx);

                    new_idx
                }
            }
        }
        self.nodes[current].value = Some(value);
    }

    pub fn build_failures(&mut self) {
        let mut queue = VecDeque::<usize>::new();
        let root = 0;
        let root_children_idxs: Vec<usize> = self.nodes[root].children.values().copied().collect();

        for c_idx in root_children_idxs {
            self.nodes[c_idx].fail = root; // all immediate child of root fails to root
            queue.push_back(c_idx); // initialize BFS traversal
        }

        while let Some(v) = queue.pop_front() {
            let v_fail = self.nodes[v].fail;
            let transitions: Vec<(String, usize)> = self.nodes[v]
                .children
                .iter()
                .map(|(k, &u)| (k.clone(), u))
                .collect();

            // finding failure link for u
            for (label, u) in transitions {
                let mut f = v_fail; // initialize failure value for u, we set it the same as v

                while f != root && !self.nodes[f].children.contains_key(&label) {
                    f = self.nodes[f].fail;
                }

                if let Some(&next) = self.nodes[f].children.get(&label) {
                    self.nodes[u].fail = next;
                } else {
                    self.nodes[u].fail = root;
                }

                queue.push_back(u);
            }
        }
    }

    pub fn estimate(&self, input_string: &String) -> Result<f32> {
        let input_tokenized = tokenize(input_string);
        println!("{:?}", input_tokenized);
        Ok(6.7)
    }

    pub fn debug_print(&self) {
        self.debug_print_recurse(0, 0);
    }

    fn debug_print_recurse(&self, index: usize, indent: usize) {
        let node = &self.nodes[index];
        for (label, &child_idx) in node.children.iter() {
            for _ in 0..indent {
                print!("  ");
            }
            print!("{}  [fail={}]", label, self.nodes[child_idx].fail);

            if let Some(v) = self.nodes[child_idx].value {
                print!("  [value = {}]", v);
            }

            println!();
            self.debug_print_recurse(child_idx, indent + 1);
        }
    }
}
