use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Result};

/// 获取文件内容
pub fn get_file_contents(path: &str) -> Result<String> {
    // let current_dir = std::env::current_dir()?;
    // let current_path = current_dir.join(path);
    // println!("current_path:{:?}", current_path);
    let current_path = format!(".{}", path);
    println!("current_path:{:?}", current_path);
    let file = File::open(current_path)?;
    println!("file:{:?}", file);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
