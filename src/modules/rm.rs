use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    let mut recursive = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    for path_str in paths {
        let path = Path::new(path_str);

        
        if path_str == "." || path_str == ".." {
            return Err(format!("rm: refusing to remove '.' or '..' directory: skipping '.'"));
        }

        if !path.exists() {
            return Err(format!("rm: cannot remove '{}': No such file or directory", path_str));
        }

        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    return Err(format!("rm: failed to remove directory '{}': {}", path_str, e));
                }
            } else {
                return Err(format!("rm: cannot remove '{}': Is a directory", path_str));
            }
        } else if let Err(e) = fs::remove_file(path) {
            return Err(format!("rm: failed to remove file '{}': {}", path_str, e));
        }
    }

    Ok(String::new())
}
