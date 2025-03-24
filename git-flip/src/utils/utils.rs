pub fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}
