pub fn tokenizer(cmd : &str) -> Result<Vec<String>,String> {
    // println!("tokenizer: {}",cmd);
    let mut tokens : Vec<String> = Vec::new();
    let mut string_to_push = String::new();
    let mut is_single = false;
    let mut is_duble = false;
    let mut is_esc = false;

    for s in cmd.chars(){
        if is_esc{
            string_to_push.push(s);
            is_esc = false;
            continue;
        }

        match s {
            '\\' => is_esc = true,
            '\'' => is_single = !is_single,
            '"' => is_duble = !is_duble,
            s if s.is_whitespace() && !is_duble && !is_single => { 
                    if !string_to_push.is_empty() {
                    tokens.push(string_to_push.clone());
                    string_to_push.clear();
                }
            }
            _ => string_to_push.push(s),
        }
    }
    if is_duble || is_single {
        return Err("You have problem with single or double quote".to_string());
    }
    if is_esc{
        return Err("You have problem with escape".to_string());
    }
    if !string_to_push.is_empty(){
        tokens.push(string_to_push);
    }
    // if tokens.is_empty(){
    //     return Err("Write Your Command".to_string());
    // }
    Ok(tokens)
}