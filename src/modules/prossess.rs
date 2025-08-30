use crate::modules::cat::cat;
use crate::modules::cp::cp;
use crate::modules::echo::echo;
use crate::modules::ls::ls;
use crate::modules::mkdir::mkdir;
use crate::modules::pwd::pwd;
// use libc::mkdir;

pub fn prossess(value: Vec<String>) -> Result<String, String> {
    let cmd = value[0].clone();
    match cmd.as_str() {
        "ls" => match ls(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "mkdir" => match mkdir(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "pwd" => match pwd(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "cat" => match cat(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "cp" => match cp(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "echo" => match echo(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        /*"rm" => Ok("exit".to_string()),
        "cd" => Ok("exit".to_string()),
        "mv" => Ok("exit".to_string()),*/
        "exit" => Ok("exit".to_string()),
        _ => Err("Write a valid Command".to_string()),
    }
}
