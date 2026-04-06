pub fn get_env_or_secret(key: &str) -> Result<String, std::env::VarError> {
    match env::var(key) {
        Ok(var) => Ok(var),
        Err(_) => match env::var(format!("{}_PATH", key)) {
            Ok(path) =>  std::fs::read_to_string(path).map_err(|_| std::env::VarError::NotPresent),
            Err(e) => Err(e)
        },
    }
}