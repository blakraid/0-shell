use crate::modules::cat::cat;
use crate::modules::cp::cp;
use crate::modules::echo::echo;
use crate::modules::ls::ls;
use crate::modules::mkdir::mkdir;
use crate::modules::pwd::pwd;
use crate::modules::rm::rm;
use crate::modules::cd::cd;
use crate::modules::mv::mv;

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
        "rm" => match rm(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "cd" => match cd(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "mv" => match mv(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },

        "exit" => Ok("exit".to_string()),
        _ => Err("Write a valid Command".to_string()),
    }
}
