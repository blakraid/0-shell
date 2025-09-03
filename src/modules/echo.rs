pub fn echo(raw_args: &str) -> Result<String, String> {
    if raw_args.is_empty() {
        return Ok("\n".to_string());
    }

    let (no_newline, content) = if raw_args.starts_with("-n ") {
        (true, &raw_args[3..])
    } else if raw_args == "-n" {
        (true, "")
    } else {
        (false, raw_args)
    };

    let output = if content.starts_with('"') && content.ends_with('"') && content.len() >= 2 {
        &content[1..content.len() - 1]
    } else if content.starts_with('\'') && content.ends_with('\'') && content.len() >= 2 {
        &content[1..content.len() - 1]
    } else {
        &escape(content)
    };

    if no_newline {
        Ok(output.to_string())
    } else {
        Ok(format!("{}\n", output))
    }
}
fn escape(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    _ => {
                        chars.next();
                        result.push(next_ch);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}
