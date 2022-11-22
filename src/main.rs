use std::env;

mod git_op;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: git op -> scan current directory -> convert -> store
}
