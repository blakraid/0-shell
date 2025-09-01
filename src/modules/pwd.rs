use std::env;

pub fn pwd(args: &[String]) -> Result<String, String> {
    if !args.is_empty() {
        return Err("pwd: too many arguments".to_string());
    }

    match env::current_dir() {
        Ok(path) => {
            match path.to_str() {
                Some(path_str) => Ok(format!("{}\n",path_str.to_string())),
                None => Err("pwd: current directory path contains invalid UTF-8".to_string()),
            }
        }
        Err(e) => Err(format!("pwd: error getting current directory: {}", e)),
    }
}
