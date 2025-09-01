use {
    std::fs,
    std::os::unix::fs::{PermissionsExt,MetadataExt},
    libc::{getpwuid, getgrgid},
    std::ffi::CStr,
    std::time::{SystemTime},
    chrono::{DateTime, Local},
    std::env,
    std::path::PathBuf,
};

struct dir_e{
    name:String,
    path: PathBuf,
}
pub fn ls(arr : &[String]) -> Result<String,String>{
    let mut tag_l = false;
    let mut tag_a = false;
    let mut tag_f = false;
    let mut number_files = 0;
    let mut vars = Vec::new();

    for op in arr{
        if op.starts_with('-'){
            for i in op.chars().skip(1){
                if i == 'l'{
                    tag_l = true;
                }else if i == 'a'{
                    tag_a = true;
                }else if i == 'F'{
                    tag_f = true;
                }else{
                    return Err("ls: invalid option".to_string());
                }
            }
            if !tag_a && !tag_f && !tag_l{
                return Err("ls: cannot access '-': No such file or directory".to_string());
            }
        }else {
            vars.push(fix_path(op));
        }
    }
    if vars.is_empty(){
        vars.push(".".to_string());
    }
    let mut result: Vec<String> = Vec::new();
    let mut ne_line = false;
    vars.sort();
    for var in &vars{
        if vars.len() != 1 {
            if ne_line {
                result.push(format!("{}:",fix_path(&var)));
            }else{
                result.push(format!("{}:",fix_path(&var)));
            }
            ne_line = true;
        }
        match fs::read_dir(&var){
            Ok(files) => {
                let mut entries: Vec<dir_e> = files.filter_map(|e| {
                    e.ok().map(|entry| {
                        let path = entry.path();
                        let name = match path.file_name() {
                            Some(v) => v.to_string_lossy().to_string(),
                            None => String::new(),
                        };
                        dir_e {
                            name: name,
                            path: path,
                        }
                    })
                }).collect();

                entries.sort_by(|a, b| {
                        let a_name = a.name.trim_matches('.').to_lowercase();
                        let b_name = b.name.trim_matches('.').to_lowercase();
                        a_name.cmp(&b_name)
                });

                entries.insert(0, dir_e { name: ".".to_string(), path: PathBuf::from(var) });
                entries.insert(1, dir_e { name: "..".to_string(), path: PathBuf::from(format!("{}/..",var)) });

                
                
                let mut push_result: String = String::new();
                for read_file in entries{
                            let path = read_file.path;
                            let name_file = read_file.name;

                            if !tag_a && name_file.starts_with('.'){
                                continue;
                            };
                            
                            match fs::symlink_metadata(path){
                                Ok(v) => {
                                    let mut name_display = name_file.clone();
                                    number_files+= v.blocks();
                                    if tag_f {
                                        if v.is_dir() {
                                            name_display.push('/');
                                        } else if v.file_type().is_symlink() {
                                            name_display.push('@');
                                        } else if v.permissions().mode() & 0o111 != 0 {
                                            name_display.push('*');
                                        }
                                    };
                                    if tag_l {
                                        let mode = v.permissions().mode();
                                    let perms = format!(
                                        "{}{}{}{}{}{}{}{}{}{}",
                                        if v.is_dir() {"d"} else {"-"},
                                        if mode & 0o400 != 0 { "r" } else { "-" },
                                        if mode & 0o200 != 0 { "w" } else { "-" },
                                        if mode & 0o100 != 0 { "x" } else { "-" },
                                        if mode & 0o040 != 0 { "r" } else { "-" },
                                        if mode & 0o020 != 0 { "w" } else { "-" },
                                        if mode & 0o010 != 0 { "x" } else { "-" },
                                        if mode & 0o004 != 0 { "r" } else { "-" },
                                        if mode & 0o002 != 0 { "w" } else { "-" },
                                        if mode & 0o001 != 0 { "x" } else { "-" },
                                    );

                                        let size = v.len();
                                        let nlink = v.nlink();
                                        let uid = v.uid();
                                        let get_uid = get_uid_name(uid);
                                        let gid = v.gid();
                                        let get_git = get_gid_name(gid);
                                        let mtime: DateTime<Local> = DateTime::from(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(v.mtime() as u64));
                                        let time_str = mtime.format("%b %e %H:%M").to_string();

                                        push_result.push_str(&format!(
                                            "{} {:>3} {:>5} {:>5} {:>8} {} {}\n",
                                            perms, nlink, get_uid, get_git, size, time_str, name_display
                                        ));
                                    }else {
                                        result.push(name_display);
                                    }
                                },
                                Err(_) => continue
                            };
                }
                result.push(format!("total {}\n{}",number_files/2,push_result));
                number_files = 0;

            },
            Err(_) => { return Err(format!("ls: cannot access '{}'", var));}
        }
    }
    let output = format!("{}\n",result.join("\n").trim_matches('\n').to_string());
    Ok(output)
}



fn get_uid_name(uid: u32) -> String {
    unsafe {
        let pw = getpwuid(uid);
        if pw.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*pw).pw_name);
        name.to_string_lossy().into_owned()
    }
}

fn get_gid_name(gid: u32) -> String {
    unsafe {
        let gr = getgrgid(gid);
        if gr.is_null() {
            return gid.to_string();
        }
        let name = CStr::from_ptr((*gr).gr_name);
        name.to_string_lossy().into_owned()
    }
}


fn fix_path(path: &str) -> String {
    if path.starts_with("~") {
        if path == "~" {
            return env::var("HOME").unwrap_or(".".to_string());
        } else if path.starts_with("~/") {
            let home = env::var("HOME").unwrap_or(".".to_string());
            let mut expanded = PathBuf::from(home);
            expanded.push(&path[2..]);
            return expanded.to_string_lossy().into_owned();
        }
    }
    path.to_string()
}