use {
    std::fs,
    std::os::unix::fs::{PermissionsExt,MetadataExt},
    libc::{getpwuid, getgrgid},
    std::ffi::CStr,
    std::time::{SystemTime},
    chrono::{DateTime, Local},

};

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
        }else {
            vars.push(op.clone());
        }
    }
    if vars.is_empty(){
        vars.push(".".to_string());
    }
    let mut result: Vec<String> = Vec::new();
    for var in vars{
        match fs::read_dir(&var){
            Ok(files) => {
                for file in files{
                    match file {
                        Ok(read_file) => {
                            let path = read_file.path();
                            let name_file = match path.file_name() {
                                Some(v) => v.to_string_lossy().to_string(),
                                None => continue,
                            };
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

                                        result.push(format!(
                                            "{} {:>3} {:>5} {:>5} {:>8} {} {}",
                                            perms, nlink, get_uid, get_git, size, time_str, name_display
                                        ));
                                    }else {
                                        result.push(name_display);
                                    }
                                },
                                Err(_) => continue
                            };
                            
                        },
                        Err(_) => continue
                    }
                }
            },
            Err(_) => { return Err(format!("ls: cannot access '{}'", var));}
        }
    }
    let elem = format!("total {}",number_files);
    result.insert(0, elem);
    let output = result.join("\n");
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