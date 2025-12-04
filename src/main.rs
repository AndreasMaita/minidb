use btree::node::{BPlusTree, LeafNode, Node};
use eframe::egui;

mod btree;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "minidb",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    tree: BPlusTree<String>,
    value_input: String,
    key_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let root = Node::Leaf(LeafNode::new(Vec::new()));
        Self {
            tree: BPlusTree::new(3, root),
            value_input: String::new(),
            key_input: String::new(),
        }
    }
}

// write some good comments
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌳 B+ Tree Visualizer");

            ui.horizontal(|ui| {
                ui.label("Value: ");
                ui.text_edit_singleline(&mut self.value_input);
            });

            if ui.button("➕ Insert").clicked() {
                let value = std::mem::take(&mut self.value_input);
                self.tree.insert_value(value);
                self.value_input.clear();
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Search key: ");
                ui.text_edit_singleline(&mut self.key_input);
            });
            if ui.button("Search").clicked() {
                let key = std::mem::take(&mut self.key_input);
                let value = self.tree.get(key.parse::<u8>().unwrap());

                match value {
                    None => self.key_input = "nothing found!".to_string(),
                    Some(val) => self.key_input = val.to_string(),
                }
            }

            ui.heading("Tree Structure:");

            egui::ScrollArea::both().show(ui, |ui| {
                self.display_tree_node(ui, &self.tree.root, 0);
            });
        });
    }
}

impl MyApp {
    fn display_tree_node(&self, ui: &mut egui::Ui, node: &Node<String>, level: usize) {
        match node {
            Node::Internal(internal) => {
                ui.vertical(|ui| {
                    // Draw the internal node as a box
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(100, 100, 100))
                        .show(ui, |ui| {
                            ui.label(format!("Internal: {:?}", internal.keys));
                        });

                    // Draw children horizontally
                    ui.horizontal(|ui| {
                        for (i, child) in internal.children.iter().enumerate() {
                            ui.vertical(|ui| {
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
