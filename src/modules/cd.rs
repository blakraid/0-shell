use std::env;
use std::path::Path;

pub fn cd(args: &[String]) -> Result<String, String> {
    let target = if args.is_empty() {
        env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
    } else {
        args[0].clone()
    };

    let path = Path::new(&target);

    if !path.exists() {
        return Err(format!("cd: {}: No such file or directory", target));
    }
    if !path.is_dir() {
        return Err(format!("cd: {}: Not a directory", target));
    }

    if let Err(e) = env::set_current_dir(path) {
        return Err(format!("cd: failed to change directory: {}", e));
    }

    Ok("".to_string())
}
