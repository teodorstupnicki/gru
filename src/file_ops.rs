use std::{fs, path::PathBuf};
use dirs;

pub fn parse_file(config_line: &str, ins_fi: &mut i32) {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));
    let mut home_str = home_dir.into_os_string().into_string().unwrap();
    let mut split = config_line.split(':');
    let mut system_path = split.next().unwrap().to_string();
    system_path.remove(0);
    home_str.push_str(&system_path);
    println!("File system file: {}", home_str);
    let repo_path = split.next().unwrap(); 
    println!("Repository file: {}", repo_path);
    
    let system_file_handle = fs::metadata(&home_str);
    let repo_file_handle = fs::metadata(repo_path);
    
    match system_file_handle {
        Ok(_) => *ins_fi += 1,
        Err(_) => {}
    }
    
    match repo_file_handle {
        Ok(_) => println!("File {} already exists!", repo_path),
        Err(error) => { 
            println!("Repository error: {}", error);
            println!("Repository is missing files referenced in config file! exiting");
        }
    }
}
