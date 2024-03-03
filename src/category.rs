use log::{debug, error};

pub fn check(category: &str) -> bool {
    // Match the category value
    return match category {
        "neko" => {
            debug!("Category 'neko' is valid");
            true
        }
        "kitsune" => {
            debug!("Category 'kitsune' is valid");
            true
        }
        "waifu" => {
            debug!("Category 'waifu' is valid");
            true
        }
        _ => {
            error!("Category '{}' is invalid", category);
            false
        }
    }
}