pub mod models;

use crate::models::row::MyRow;
use eframe::egui;
use minidb_engine::models::{BPlusTree, KeySize, LeafNode, Node};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "minidb",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    tree: BPlusTree<MyRow>,
    value_input: String,
    key_input: String,
    update_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let root = Node::Leaf(LeafNode::new(Vec::new()));
        Self {
            tree: BPlusTree::new(2, 4, root, Vec::new()),
            value_input: String::new(),
            key_input: String::new(),
            update_input: String::new(),
        }
    }
}

// write some good comments
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("B+ Tree Visualizer");

                ui.horizontal(|ui| {
                    ui.label("Value: ");
                    ui.text_edit_singleline(&mut self.value_input);

                    if ui.button("➕ Insert").clicked() {
                        let value = std::mem::take(&mut self.value_input);
                        self.tree.insert_value(MyRow {
                            name: (value),
                            age: (5),
                        });
                        self.value_input.clear();
                    }
                    ui.label("Search key: ");
                    ui.text_edit_singleline(&mut self.key_input);
                    if ui.button("Search").clicked() {
                        let key = std::mem::take(&mut self.key_input);
                        let value = self.tree.get(key.parse::<KeySize>().unwrap());

                        match value {
                            None => self.key_input = "nothing found!".to_string(),
                            Some(val) => self.key_input = val.to_string(),
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("replace key: ");
                    ui.text_edit_singleline(&mut self.key_input);
                    ui.label("with Value: ");
                    ui.text_edit_singleline(&mut self.update_input);
                    if ui.button("update").clicked() {
                        let key = std::mem::take(&mut self.key_input);
                        let obj = MyRow {
                            name: self.update_input.clone(),
                            age: 0,
                        };

                        self.tree
                            .update(key.parse::<KeySize>().unwrap(), obj.clone());
                    }
                });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("delete key: ");
                    ui.text_edit_singleline(&mut self.key_input);

                    if ui.button("delete").clicked() {
                        let key = std::mem::take(&mut self.key_input);
                        self.tree.delete_value(key.parse::<KeySize>().unwrap());
                    }
                });
                ui.separator();
                ui.heading("Tree Structure:");
                egui::ScrollArea::both().show(ui, |ui| {
                    self.display_tree_node(ui, &self.tree.root, 0);
                });
            });
        });
    }
}

impl MyApp {
    fn display_tree_node<V>(&self, ui: &mut egui::Ui, node: &Node<V>, level: usize)
    where
        V: ToString + std::fmt::Debug,
    {
        match node {
            Node::Internal(internal) => {
                ui.horizontal(|ui| {
                    // Draw the internal node as a box
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(150, 150, 255))
                        .show(ui, |ui| {
                            ui.label(format!("Internal: {:?}", internal.keys));
                        });

                    // Draw children horizontally
                    ui.vertical(|ui| {
                        for (i, child) in internal.children.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("↓ {}", i));
                                self.display_tree_node(ui, child, level + 1);
                            });
                        }
                    });
                });
            }
            Node::Leaf(leaf) => {
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(150, 200, 150))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(format!("Leaf"));
                            ui.label(format!("keys: {:?}", leaf.keys));
                            ui.label(format!("values: {:?}", leaf.values));
                        });
                    });
            }
        }
    }
}
