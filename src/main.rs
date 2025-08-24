use std::collections::HashMap;

mod binary_search;
mod linear_search;

use binary_search::binary_search;
use linear_search::linear_search;

// A struct to demonstrate searching with a custom predicate.
#[derive(Debug, PartialEq)]
struct Product {
    id: u32,
    name: String,
    in_stock: bool,
}

fn main() {
    // --- Manual Implementations ---
    println!("--- Custom Search Implementations ---");
    let numbers = vec![1, 5, 10, 15, 20, 25, 30];
    let target = 20;

    println!("Searching for {} in {:?}", target, numbers);

    // Linear Search (works on unsorted data too)
    match linear_search(&target, &numbers) {
        Some(index) => println!("Linear Search: Found at index {}", index),
        None => println!("Linear Search: Not found"),
    }

    // Binary Search (requires sorted data)
    match binary_search(&target, &numbers) {
        Some(index) => println!("Binary Search: Found at index {}", index),
        None => println!("Binary Search: Not found"),
    }

    // --- Using Rust's Built-in Methods ---
    println!("\n--- Rust's Built-in Methods ---");
    let sorted_data = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let target_std = 13;

    // The idiomatic way to do a binary search.
    match sorted_data.binary_search(&target_std) {
        Ok(index) => println!("std::binary_search: Found {} at index {}", target_std, index),
        Err(insertion_point) => {
            println!("std::binary_search: {} not found. Should be inserted at {}", target_std, insertion_point)
        }
    }

    // --- Searching a Vector of Structs with `find()` ---
    println!("\n--- Searching a Vector of Structs ---");
    let products = vec![
        Product { id: 101, name: "Laptop".to_string(), in_stock: true },
        Product { id: 102, name: "Mouse".to_string(), in_stock: false },
        Product { id: 103, name: "Keyboard".to_string(), in_stock: true },
    ];
    let search_id = 102;

    // `find()` takes a closure to define the search condition.
    let found_product = products.iter().find(|p| p.id == search_id);

    match found_product {
        Some(product) => println!("`find()`: Found product: {:?}", product),
        None => println!("`find()`: Product with id {} not found.", search_id),
    }

    // --- O(1) Search with HashMap ---
    println!("\n--- O(1) Search with HashMap ---");
    let mut user_scores = HashMap::new();
    user_scores.insert("alice", 95);
    user_scores.insert("bob", 82);
    user_scores.insert("charlie", 100);

    let user_to_find = "bob";
    match user_scores.get(user_to_find) {
        Some(score) => println!("HashMap `get()`: Score for {} is {}", user_to_find, score),
        None => println!("HashMap `get()`: User {} not found.", user_to_find),
    }
}
