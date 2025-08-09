use crate::modules::ls::ls;

pub fn prossess(value : Vec<String>) -> Result<String,String>{

    let cmd = value[0].clone();
    match cmd.as_str() {
        "ls" => match ls(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        /*"rm" => Ok("exit".to_string()),
        "echo" => Ok("exit".to_string()),
        "cd" => Ok("exit".to_string()),
        "pwd" => Ok("exit".to_string()),
        "cat" => Ok("exit".to_string()),
        "cp" => Ok("exit".to_string()),
        "mv" => Ok("exit".to_string()),
        "mkdir" => Ok("exit".to_string()),*/
        "exit" => Ok("exit".to_string()),
        _ => Err("Write a valid Command".to_string())
        }
    }