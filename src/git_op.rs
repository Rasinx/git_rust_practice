use std::{fmt::format, env, path::Path, fs::File, io::Write, f64::consts::E};

use crate::utils::{zlib_encode, sha1_encode};

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
        let file_path_str = format!("./objects/{}/{}", &hash_code[..2], &hash_code[2..]);
        let file_path = Path::new(&file_path_str);
        match file_path.exists() {
            true => print!("{file_path_str} is exists!"),
            false => {
                let parent_dir = file_path.parent().unwrap();
                File::create(&parent_dir).unwrap();
            },
        }
        let mut file = File::create(&file_path).unwrap();
        match file.write_all(&final_ctx) {
            Ok(_) => Ok(file_path_str),
            Err(err) => Err(err.to_string()),
        } 
    }
}


#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_git_rust() {

    }
}