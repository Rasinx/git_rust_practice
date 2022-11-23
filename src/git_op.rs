use std::{fmt::format, env, path::Path, fs::{File, self}, io::{Write, Read}, f64::consts::E};

use crate::utils::{zlib_encode, sha1_encode, zlib_decode};

enum Command {
    Add,
    Cat,
    Commit,
}

struct GitBaseObj {
    obj_type: String,
    content: Vec<u8>,
    src_name: String,
}

impl GitBaseObj {
    pub fn new(obj_type: String, content: Vec<u8>, src_name: String) -> Self {
        Self { obj_type, content, src_name } 
    }
    pub fn store_self(&self) -> Result<String, String>{
        let ctx_str = String::from_utf8(self.content.clone()).unwrap();
        let git_obj_ctx = format!("{} {} {}", self.obj_type, self.content.len(), ctx_str);
        let final_ctx = zlib_encode(&git_obj_ctx);
        let hash_code = sha1_encode(&self.content);
        // store
        let root_path = env::current_dir().expect("current_dir env path fail.");
        let curr_dir = root_path.to_str().expect("env path fail.");
        let file_path_str = format!("{}\\objects\\{}\\{}", curr_dir, &hash_code[..2], &hash_code[2..]);
        let file_path = Path::new(&file_path_str);
        match file_path.exists() {
            true => print!("{file_path_str} is exists!"),
            false => {
                let parent_dir = file_path.parent().expect("Executable must be in some directory");
                println!("{:?}", parent_dir);
                fs::create_dir_all(&parent_dir).unwrap();
            },
        }
        let mut file = File::create(&file_path).unwrap();
        match file.write_all(&final_ctx) {
            Ok(_) => Ok(file_path_str),
            Err(err) => Err(err.to_string()),
        } 
    }
}

fn cat_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut f = File::open(path).unwrap();
    let mut content:Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut content);

    zlib_decode(&content)
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_git_rust() {
        let s = String::from("version1").into_bytes();
        let test001 = GitBaseObj::new("blob".to_string(), s, "test001".to_string());
        let s_rst = test001.store_self();
        println!("{:?}", s_rst);
        let src_content = cat_file(&s_rst.unwrap());
        println!("{:?}", src_content);
    }
}