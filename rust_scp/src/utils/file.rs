pub mod utils;
use std::fs;
use std::io::{Write,Read};
pub struct file_info {
    md5sum:u64,
    size:u64,
    name:String,
    reader:read::Read,
}

pub fn new(name:String,size:u64, md5sum:u64) -> file_info {
    file_info {
        name:name,
        size:size,
        md5sum:md5sum,
        reader:read::new(name),
    }
}