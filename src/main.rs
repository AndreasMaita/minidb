use eframe::egui;
use bplustree::{BPlusTree, Node, InternalNode};

mod bplustree;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "B+ Tree Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    tree: BPlusTree<i32, String>,
    key_input: String,
    value_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let root = Node::Internal(InternalNode::new(vec![1,2,3]));
        Self {
            tree: BPlusTree::new(3, root),
            key_input: String::new(),
            value_input: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌳 B+ Tree Visualizer");
            
            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.key_input);
            });
            
            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.value_input);
            });
            
            if ui.button("➕ Insert").clicked() {
                if let Ok(key) = self.key_input.parse::<i32>() {
                    self.tree.insert(key, self.value_input.clone());
                    self.key_input.clear();
                    self.value_input.clear();
                }
            }
            
            ui.separator();
            ui.heading("Tree Structure:");
            
            egui::ScrollArea::both().show(ui, |ui| {
                self.display_tree_node(ui, &self.tree.root, 0);
            });
        });
    }
}

// THIS WAS MISSING - The implementation of display_tree_node
impl MyApp {
    fn display_tree_node(&self, ui: &mut egui::Ui, node: &Node<i32, String>, level: usize) {
        match node {
            Node::Internal(internal) => {
                ui.vertical(|ui| {
                    // Draw the internal node as a box
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(100, 150, 200))
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
                            ui.label(format!("V: {:?}", leaf.values));
                        });
                    });
            }
        }
    }
}

