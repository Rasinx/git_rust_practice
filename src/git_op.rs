use std::{env, path::Path, fs::{File, self}, io::{Write, Read}};

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
        if let Err(err) = file.write_all(&final_ctx) {
            Err(err.to_string())
        } else {
            Result::Ok(file_path_str)
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

fn add_op(args: Vec<String>) -> Result<Vec<String>, String> {
    let mut store_file_vec: Vec<String> = Vec::new();
    if args.len()==0 || args[0] == "." {
        let dir = fs::read_dir(".").expect("get directory fail");
        for ele in dir {
            if let Ok(e) = ele {
                let e_path = e.path();
                // TODO: check ignore directory and file path;
                let file_name = e.file_name().into_string().expect("get file name fail");
                let git_obj: GitBaseObj = if e_path.is_dir() {
                    // tree file
                    mk_tree_file(&file_name)
                }else {
                    // blob file
                    mk_blob_file(&file_name)
                };
                let store_rst = git_obj.store_self().expect("store self fail");
                store_file_vec.push(store_rst.clone());
            }
        }
    }
    Ok(store_file_vec)
}

fn mk_tree_file(dir_name: &str) -> GitBaseObj {
    let children = fs::read_dir(&format!("./{}", dir_name));
    for ele in children {
        // TODO: i need a recursion.
        // * tree obj
        //  * ------------------------------------------------
        //  * tree hash_code origin_name   // this depends on their children
        //  * blob hash_code origin_name
        //  * ------------------------------------------------
        // 
    }
    todo!()
}

fn mk_blob_file(file_name: &String) -> GitBaseObj {
    let mut con_vec: Vec<u8> = Vec::new();
    let mut file = File::open(&file_name).unwrap();
    let _ = file.read_to_end(&mut con_vec).expect("read file fail");
    let git_obj = GitBaseObj::new("blob".into(), con_vec, file_name.clone());
    git_obj
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

    #[test]
    fn test_git_add() {
        let add_rst = add_op(Vec::new()).expect("add op fail");
        for ele in add_rst.iter() {
            let tmp_ctx = cat_file(&ele[..]);
            println!("{tmp_ctx}");
        }
    }
}