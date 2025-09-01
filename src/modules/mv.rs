use std::fs;
use std::path::Path;

pub fn mv(args: &[String]) -> Result<String, String> {
    if args.len() != 2 {
        return Err("mv: missing file operand or destination".to_string());
    }

    let src = Path::new(&args[0]);
    let dest = Path::new(&args[1]);

    if !src.exists() {
        return Err(format!("mv: cannot stat '{}': No such file or directory", args[0]));
    }

    if let Err(e) = fs::rename(src, dest) {
        return Err(format!("mv: failed to move '{}': {}", args[0], e));
    }

    Ok("".to_string())
}
