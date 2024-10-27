use ic_cdk_macros::{update, query};
use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::collections::HashMap;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

/// Represents an item in the supermarket's inventory
#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct InventoryItem {
    pub id: u32,                // Unique ID for the item
    pub name: String,           // Name of the item
    pub quantity: u32,          // Quantity of the item in stock
    pub price: f64,             // Price of the item
    pub expiration_date: u64,   // Expiration date of the item as a Unix timestamp
}

/// Manages the supermarket inventory and keeps a log of changes
pub struct SupermarketManager {
    pub items: HashMap<u32, InventoryItem>, // HashMap to store items by their ID
    pub logs: Vec<String>,                  // Vector to keep logs of all changes made to inventory
}

impl SupermarketManager {
    /// Initializes a new SupermarketManager with an empty inventory and log
    pub fn new() -> Self {
        SupermarketManager {
            items: HashMap::new(),
            logs: Vec::new(),
        }
    }

    /// Helper function to get the current timestamp in RFC3339 format as a string
    /// This is used to log the exact time of changes made to the inventory
    pub fn get_current_time() -> String {
        let now = OffsetDateTime::now_utc();
        now.format(&Rfc3339).unwrap()  // Formats the current time in a readable format
    }

    /// Adds a new item to the inventory
    /// - `item`: The item to add
    pub fn add_item(&mut self, item: InventoryItem) {
        self.items.insert(item.id, item.clone()); // Add the item to the inventory HashMap
        let log = format!(
            "Item {} added at {}",
            item.id,
            SupermarketManager::get_current_time()
        );
        self.logs.push(log); // Log the addition with the current timestamp
    }

    /// Retrieves an item from the inventory by ID
    /// - `id`: The ID of the item to retrieve
    /// Returns an Option<&InventoryItem> which is Some if the item exists, or None if it doesn't
    pub fn get_item(&self, id: u32) -> Option<&InventoryItem> {
        self.items.get(&id) // Lookup the item by ID in the HashMap
    }

    /// Updates the quantity of an existing item in the inventory
    /// - `id`: The ID of the item to update
    /// - `quantity`: The new quantity of the item
    pub fn update_item_quantity(&mut self, id: u32, quantity: u32) {
        if let Some(item) = self.items.get_mut(&id) { // Check if the item exists
            item.quantity = quantity; // Update the quantity
            let log = format!(
                "Item {} quantity updated to {} at {}",
                id,
                quantity,
                SupermarketManager::get_current_time()
            );
            self.logs.push(log); // Log the update with the current timestamp
        }
    }

    /// Removes an item from the inventory by ID
    /// - `id`: The ID of the item to remove
    pub fn remove_item(&mut self, id: u32) {
        if self.items.remove(&id).is_some() { // Remove the item if it exists
            let log = format!(
                "Item {} removed at {}",
                id,
                SupermarketManager::get_current_time()
            );
            self.logs.push(log); // Log the removal with the current timestamp
        }
    }

    /// Retrieves all logs of changes made to the inventory
    /// Returns a vector of strings, each representing a log entry
    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone() // Return a copy of the logs
    }
}


use std::cell::RefCell;

// Create a thread-local variable for the SupermarketManager.
// This allows the state to be persisted within the canister.
thread_local! {
    static INVENTORY_MANAGER: RefCell<SupermarketManager> = RefCell::new(SupermarketManager::new());
}

// Adds a new item to the inventory.
// This function is marked as `#[update]` because it modifies state.
#[update]
fn add_inventory_item(id: u32, name: String, quantity: u32, price: f64, expiration_date: u64) {
    let item = InventoryItem {
        id,
        name,
        quantity,
        price,
        expiration_date,
    };

    INVENTORY_MANAGER.with(|inventory| {
        inventory.borrow_mut().add_item(item);
    });
}

// Retrieves an item by ID.
// This function is marked as `#[query]` because it only reads state and does not modify it.
#[query]
fn get_inventory_item(id: u32) -> Option<InventoryItem> {
    INVENTORY_MANAGER.with(|inventory| {
        inventory.borrow().get_item(id).cloned()
    })
}

// Updates the quantity of an existing item in the inventory.
// This function is marked as `#[update]` because it modifies state.
#[update]
fn update_inventory_quantity(id: u32, quantity: u32) {
    INVENTORY_MANAGER.with(|inventory| {
        inventory.borrow_mut().update_item_quantity(id, quantity);
    });
}

// Removes an item from the inventory by ID.
// This function is marked as `#[update]` because it modifies state.
#[update]
fn remove_inventory_item(id: u32) {
    INVENTORY_MANAGER.with(|inventory| {
        inventory.borrow_mut().remove_item(id);
    });
}

// Retrieves all logs of changes made to the inventory.
// This function is marked as `#[query]` because it only reads state.
#[query]
fn get_inventory_logs() -> Vec<String> {
    INVENTORY_MANAGER.with(|inventory| {
        inventory.borrow().get_logs()
    })
}
