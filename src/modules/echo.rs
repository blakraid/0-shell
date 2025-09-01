pub fn echo(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Ok(String::new());
    }

    let (no_newline, text_args) = if args[0] == "-n" {
        (true, &args[1..])
    } else {
        (false, args)
    };

    let output = text_args.join(" ");

    if no_newline {
        Ok(format!("{}\n",output))
    } else {
        if output.is_empty() {
            Ok(String::new())
        } else {
            Ok(format!("{}\n",output))
        }
    }
}
