use std::collections::{HashMap, HashSet};
use std::io;

struct SmartShoppingList {
    items: HashMap<String, usize>,
}

impl SmartShoppingList {
    fn add_item(&mut self, item: &str, quantity: usize) {
        let entry = self.items.entry(item.to_string()).or_insert(0);
        *entry += quantity;
    }

    fn remove_item(&mut self, item: &str, quantity: usize) {
        if let Some(entry) = self.items.get_mut(item) {
            if *entry >= quantity {
                *entry -= quantity;
                if *entry == 0 {
                    self.items.remove(item);
                }
            } else {
                println!("Fehler: Nicht genug {} in der Liste.", item);
            }
        } else {
            println!("Fehler: {} nicht in der Liste.", item);
        }
    }

    fn display_list(&self) {
        println!("Einkaufsliste:");
        for (item, quantity) in &self.items {
            println!("{}: {}", item, quantity);
        }
    }

    fn generate_suggestions(&self, available_items: &HashSet<String>) -> Vec<String> {
        let missing_items: Vec<&String> = self
            .items
            .keys()
            .filter(|&item| !available_items.contains(item))
            .collect();

        let mut suggestions = Vec::new();
        for item in missing_items {
            suggestions.push(format!("Kaufe {} als Alternative.", item));
        }

        suggestions
    }
}

fn main() {
    let mut shopping_list = SmartShoppingList { items: HashMap::new() };

    shopping_list.add_item("Apfel", 3);
    shopping_list.add_item("Milch", 1);
    shopping_list.add_item("Brot", 2);

    shopping_list.display_list();

    shopping_list.remove_item("Brot", 1);

    shopping_list.display_list();

    let available_items: HashSet<String> = ["Apfel", "Milch", "Ei", "Wasser"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let suggestions = shopping_list.generate_suggestions(&available_items);
    println!("Vorschläge für fehlende Artikel:");
    for suggestion in suggestions {
        println!("{}", suggestion);
    }
}
