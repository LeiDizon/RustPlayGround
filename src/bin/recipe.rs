// Import statements: bring in functionality we need
// 'serde' provides traits for serialization/deserialization (converting to/from JSON)
// 'Deserialize' converts JSON -> Rust struct, 'Serialize' converts Rust struct -> JSON
// 'fs' stands for file system - allows us to read files
use serde::{Deserialize, Serialize};
use std::fs;

// Define the data structures that match our JSON format
// The #[derive(...)] attribute automatically generates code for us
// - Serialize: allows converting Rust -> JSON
// - Deserialize: allows converting JSON -> Rust
// - Debug: allows printing with {:?}
#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    // String fields must match exactly with JSON field names
    name: String,              // The recipe's name (e.g., "Spaghetti Carbonara")
    ingredients: Vec<String>,  // A vector (list) of ingredient strings
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipeDatabase {
    // This is the top-level structure that contains our array of recipes
    // It matches the JSON structure: { "recipes": [...] }
    recipes: Vec<Recipe>,  // A vector of Recipe structs
}

fn main() {
    // === STEP 1: READ THE FILE ===
    // fs::read_to_string() reads the entire file and returns a Result<String, Error>
    // .expect() means: "If reading fails, crash with this error message"
    // We store the contents in 'file_content' variable as a String
    let file_content = fs::read_to_string("data/recipes.json")
        .expect("Failed to read data/recipes.json file");

    // === STEP 2: PARSE JSON INTO RUST STRUCTS ===
    // serde_json::from_str() converts the JSON string into our RecipeDatabase struct
    // The type annotation ': RecipeDatabase' tells Rust what type to create
    // .expect() again: if parsing fails, crash with this error message
    let database: RecipeDatabase = serde_json::from_str(&file_content)
        .expect("Failed to parse JSON");

    // === STEP 3: DISPLAY ALL RECIPES ===
    println!("=== Recipe Database ===\n");
    
    // .iter() creates an iterator (a way to loop through the vector)
    // .enumerate() gives us both the index (0, 1, 2...) and each item
    // 'for' loop: repeat the block for each recipe in the database
    for (index, recipe) in database.recipes.iter().enumerate() {
        // Print the recipe number and name
        // index starts at 0, so we add 1 to show "Recipe 1" instead of "Recipe 0"
        println!("Recipe {}: {}", index + 1, recipe.name);
        println!("Ingredients:");
        
        // Loop through each ingredient in this recipe
        // .iter().enumerate() again for the ingredients
        for (i, ingredient) in recipe.ingredients.iter().enumerate() {
            // Print each ingredient with a number (1, 2, 3...)
            println!("  {}. {}", i + 1, ingredient);
        }
        println!();  // Print a blank line for readability
    }

    // === STEP 4: SEARCH FOR A SPECIFIC RECIPE ===
    let search_name = "Chicken Stir Fry";
    
    // .iter() loops through recipes
    // .find(|r| r.name == search_name) searches for a recipe with matching name
    // The |r| is a closure (anonymous function) that checks if r.name matches
    // .find() returns an Option: Some(recipe) if found, None if not found
    if let Some(recipe) = database.recipes.iter().find(|r| r.name == search_name) {
        // This code runs only if we found a matching recipe
        println!("--- Found Recipe: {} ---", search_name);
        println!("Ingredients needed:");
        
        // Loop through this recipe's ingredients and print each one
        // &recipe.ingredients: 'ref' means "borrow" - don't take ownership
        for ingredient in &recipe.ingredients {
            println!("  ✓ {}", ingredient);
        }
    } else {
        // This runs if the recipe was NOT found
        println!("Recipe '{}' not found", search_name);
    }
}
