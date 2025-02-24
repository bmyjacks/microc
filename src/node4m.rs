use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fmt, mem};

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn increment_global_counter() -> usize {
    GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn get_global_counter() -> usize {
    GLOBAL_COUNTER.load(Ordering::SeqCst)
}

pub fn reset_global_counter() {
    GLOBAL_COUNTER.store(0, Ordering::SeqCst);
}

#[derive(Clone)]
pub struct Node {
    name: String,
    value: String,
    children: Vec<Node>,
    id: usize,
}

impl Node {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn new(name: String, value: String) -> Node {
        let id = get_global_counter();
        increment_global_counter();
        Node {
            name,
            value,
            children: Vec::new(),
            id,
        }
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn take_children(&mut self) -> Vec<Node> {
        mem::take(&mut self.children)
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn to_dot(&self, display_value: bool) -> String {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        self.to_dot_helper(&mut dot, display_value);
        dot.push_str("}\n");
        dot
    }

    fn to_dot_helper(&self, dot: &mut String, display_value: bool) {
        if display_value {
            dot.push_str(&format!(
                "    \"{}\" [label=\"{}\"];\n",
                self.id, self.value
            ));
        } else {
            dot.push_str(&format!("    \"{}\" [label=\"{}\"];\n", self.id, self.name));
        }

        for child in &self.children {
            dot.push_str(&format!("    \"{}\" -> \"{}\";\n", self.id, child.id));
            child.to_dot_helper(dot, display_value);
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("value", &self.value)
            .field("children", &self.children)
            .finish()
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(name: {}, value: {})", self.name, self.value)
    }
}
