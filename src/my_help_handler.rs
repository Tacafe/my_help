// use std::fs;
// use std::io::{self, Read};
use std::process::Command;

pub fn open_my_help_file_with(my_help_file_path: &str, app: &str) {
    let _status = Command::new(app)
        .arg(my_help_file_path)
        .status()
        .expect("Failed to open file with vim");
}

pub fn read_content(my_help_file_path: &str, keywords: Vec<String>) {
    println!(
        "keywords: {}\nmy_help_file_path: {}",
        keywords.join(" "),
        my_help_file_path
    );
    // let mut file = fs::File::open(".my_help").expect("File not found");
    // let mut content = String::new();
    // file.read_to_string(&mut content)
    //     .expect("Could not read file");
    // println!("{}", content);
}
