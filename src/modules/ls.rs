pub fn ls(arr : &[String]) -> Result<String,String>{
    if arr.is_empty(){
        return Err("error".to_string());
    }
    let mut tag_l = false;
    let mut tag_a = false;
    let mut tag_f = false;
    let mut vars = Vec::new();

    for op in arr{
        if op.starts_with('-'){
            if op.contains('l'){
                tag_l = true;
            }else if op.contains('a'){
                tag_a = true;
            }else if op.contains('f'){
                tag_f = true;
            }

        }else {
            vars.push(op);
        }
    }
}