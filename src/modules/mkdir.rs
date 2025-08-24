use std::fs;

pub fn mkdir(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".to_string());
    }

    let dir_name = &args[0];

    match fs::create_dir(dir_name) {
        Ok(_) => Ok(String::new()), 
        Err(e) => Err(format!(
            "mkdir: cannot create directory '{}': {}",
            dir_name, e
        )),
    }
}
