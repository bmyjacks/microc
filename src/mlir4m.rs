use crate::node4m::Node;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

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
pub struct Mlir4m<'a> {
    ast: &'a Node,
    id_map: HashMap<String, usize>,
}

impl<'a> Mlir4m<'a> {
    pub fn new(ast: &'a Node) -> Self {
        reset_global_counter();
        Self {
            ast,
            id_map: HashMap::new(),
        }
    }

    pub fn generate_mlir(&mut self) -> String {
        let mut mlir = String::new();
        // Print module header
        mlir.push_str("module {\n");

        mlir.push_str("  func.func private @read() -> i32\n");
        mlir.push_str("  func.func private @print(i32)\n\n");

        mlir.push_str("  func.func @main() {\n");

        // Generate MLIR from AST
        self.traverse_ast(self.ast, 4, &mut mlir);

        // Print function and module closing
        mlir.push_str("    return\n");
        mlir.push_str("  }\n");
        mlir.push_str("}\n");

        mlir
    }

    fn get_ssa(&mut self, node: &Node, increment: bool) -> (bool, usize) {
        let value = if node.name() == "PLUSOP" || node.name() == "MINUSOP" {
            node.id().to_string()
        } else {
            node.value().to_string()
        };

        let id = self.id_map.get(&value);
        match id {
            Some(id) => {
                if increment {
                    let new_id = get_global_counter();
                    increment_global_counter();
                    self.id_map.insert(value.to_owned(), new_id);

                    (false, new_id)
                } else {
                    (false, *id)
                }
            }
            None => {
                let new_id = get_global_counter();
                increment_global_counter();
                self.id_map.insert(value.to_owned(), new_id);
                (true, new_id)
            }
        }
    }

    fn traverse_ast(&mut self, node: &Node, indent: usize, mlir: &mut String) {
        let spaces = " ".repeat(indent);

        match node.name().as_str() {
            "READ" => {
                for child in node.children() {
                    let (flag, ssa) = self.get_ssa(child, false);
                    if flag {
                        mlir.push_str(&format!(
                            "{}%{} = arith.constant 0 : i32 // Declare {}\n",
                            spaces,
                            ssa,
                            child.value()
                        ));
                    }

                    mlir.push_str(&format!(
                        "{}%{} = call @read() : () -> i32\n",
                        spaces,
                        self.get_ssa(child, true).1
                    ));
                }
                mlir.push('\n');
            }
            "WRITE" => {
                for child in node.children() {
                    match child.name().as_str() {
                        "INTLITERAL" => {
                            mlir.push_str(&format!(
                                "{}%{} = arith.constant {} : i32\n",
                                spaces,
                                self.get_ssa(child, true).1,
                                child.value()
                            ));
                            mlir.push_str(&format!(
                                "{}call @print(%{}) : (i32) -> ()\n",
                                spaces,
                                self.get_ssa(child, false).1,
                            ))
                        }
                        "ID" => mlir.push_str(&format!(
                            "{}call @print(%{}) : (i32) -> ()\n",
                            spaces,
                            self.get_ssa(child, false).1
                        )),
                        _ => {
                            self.traverse_ast(child, indent, mlir);
                            mlir.push_str(&format!(
                                "{}call @print(%{}) : (i32) -> ()\n",
                                spaces,
                                self.get_ssa(child, false).1
                            ))
                        }
                    }
                }
            }
            "INTLITERAL" => {
                mlir.push_str(&format!(
                    "{}%{} = arith.constant {} : i32\n",
                    spaces,
                    self.get_ssa(node, true).1,
                    node.value()
                ));
            }
            "PLUSOP" => {
                self.traverse_ast(&node.children()[0], indent, mlir);
                self.traverse_ast(&node.children()[1], indent, mlir);

                mlir.push_str(&format!(
                    "{}%{} = arith.addi %{}, %{} : i32\n",
                    spaces,
                    self.get_ssa(node, false).1,
                    self.get_ssa(&node.children()[0], false).1,
                    self.get_ssa(&node.children()[1], false).1,
                ));
            }
            "MINUSOP" => {
                self.traverse_ast(&node.children()[0], indent, mlir);
                self.traverse_ast(&node.children()[1], indent, mlir);

                mlir.push_str(&format!(
                    "{}%{} = arith.subi %{}, %{} : i32\n",
                    spaces,
                    self.get_ssa(node, false).1,
                    self.get_ssa(&node.children()[0], false).1,
                    self.get_ssa(&node.children()[1], false).1,
                ));
            }
            "ASSIGNOP" => {
                self.traverse_ast(&node.children()[1], indent, mlir);
                mlir.push_str(&format!(
                    "{}%{} = arith.constant 0 : i32 // Declare {}\n",
                    spaces,
                    self.get_ssa(&node.children()[0], true).1,
                    node.children()[0].value()
                ));
                let (flag, old_ssa) = self.get_ssa(&node.children()[0], false);
                mlir.push_str(&format!(
                    "{}%{} = arith.addi %{}, %{} : i32\n",
                    spaces,
                    self.get_ssa(&node.children()[0], true).1,
                    old_ssa,
                    self.get_ssa(&node.children()[1], false).1
                ));

                mlir.push('\n');
            }
            _ => {
                for child in node.children() {
                    self.traverse_ast(child, indent, mlir);
                }
            }
        }
    }
}
