pub async fn check(category: &String) -> Result<String, String> {
    let category = category.to_lowercase();

    match category.as_str() {
        "neko" => Ok(category),
        "kitsune" => Ok(category),
        "waifu" => Ok(category),
        _ => {
            Err("Invalid category".to_string())
        }
    }
}
