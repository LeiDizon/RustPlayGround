// Import statements
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;  // For user input

// Define the data structures that match our JSON format
#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    name: String,
    ingredients: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipeDatabase {
    recipes: Vec<Recipe>,
}

fn main() {
    // Read and parse the JSON file
    let file_content = fs::read_to_string("data/recipes.json")
        .expect("Failed to read data/recipes.json file");

    let database: RecipeDatabase = serde_json::from_str(&file_content)
        .expect("Failed to parse JSON");

    // Ask user for input
    println!("=== Recipe Finder ===\n");
    println!("What recipe are you looking for?");
    
    let mut search_name = String::new();  // Create an empty string to store input
    io::stdin().read_line(&mut search_name)
        .expect("Failed to read input");
    
    let search_name = search_name.trim();  // Remove the newline character

    // Search for the recipe
    if let Some(recipe) = database.recipes.iter().find(|r| r.name.to_lowercase() == search_name.to_lowercase()) {
        println!("\n✓ Found Recipe: {}\n", recipe.name);
        println!("Ingredients:");
        for (i, ingredient) in recipe.ingredients.iter().enumerate() {
            println!("  {}. {}", i + 1, ingredient);
        }
    } else {
        println!("\n✗ Recipe '{}' not found", search_name);
    }
}

