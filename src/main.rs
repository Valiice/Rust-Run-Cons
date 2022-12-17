use std::fs;
use std::process::Command;

fn main() {
    let directories = fs::read_dir(".").unwrap();

    for dir in directories {
        let dir = dir.unwrap();
        let dir_name = dir.file_name();
        let dir_unwrapped = dir_name.to_str().unwrap();
        if dir_unwrapped.ends_with(".Cons") {
            Command::new("dotnet")
                .arg("watch")
                .current_dir(dir.path())
                .spawn()
                .expect("failed to execute process");
        }
    }
    println!("Wait for all the clients to run.");
    let mut line = String::new();
    let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
}