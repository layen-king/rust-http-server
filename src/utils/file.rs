use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Result};

/// 获取文件内容
pub fn get_file_contents(path: &str) -> Result<String> {
    let file = File::open(path)?;
    println!("file:{:?}",file);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
