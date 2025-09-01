use {
    crate::modules::cat::cat,
    crate::modules::cp::cp,
    crate::modules::echo::echo,
    crate::modules::ls::ls,
    crate::modules::mkdir::mkdir,
    crate::modules::pwd::pwd,
    crate::modules::rm::rm,
    crate::modules::cd::cd,
    crate::modules::mv::mv,
};

pub fn prossess(value: Vec<String>, input: &str) -> Result<String, String> {
    let cmd = value[0].clone();
    match cmd.as_str() {
        "ls" => match ls(&value[1..]) {
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
        "echo" => {
            let prcss = input.strip_prefix("echo").unwrap_or("").trim_start();
            match echo(prcss) {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            }
        },
        "exit" => Ok("exit".to_string()),
        _ => Err(format!("Command '{}' not found",cmd)),
    }
}
