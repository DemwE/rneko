use log::{debug, error};

pub async fn check(category: &String) -> Result<String, String> {
    // Firstly, convert the category to lowercase for comparison
    let category = category.to_lowercase();
    debug!("Starting the check function, category is: {}", category);

    // Match the category value
    match category.as_str() {
        "neko" => {
            debug!("Category 'neko' is valid");
            Ok(category)
        }
        "kitsune" => {
            debug!("Category 'kitsune' is valid");
            Ok(category)
        }
        "waifu" => {
            debug!("Category 'waifu' is valid");
            Ok(category)
        }
        _ => {
            // If the value is unmatched, return an error
            debug!("Category '{}' is invalid", category);
            error!("Invalid category was given");
            Err("Invalid category".to_string())
        }
    }
}